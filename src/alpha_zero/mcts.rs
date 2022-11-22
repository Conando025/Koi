use std::cell::Ref;
use super::{Config, Game, Network, Node, NodeRef};
use crate::alpha_zero::game::Player;
use rand_distr::{Distribution, Gamma};
use std::collections::HashMap;
use std::ops::Deref;

pub fn run<G: Game>(config: Config, game: &G, network: &Network<G>) -> (Option<G::Action>, NodeRef<G>) {
    let mut root = Node::new(0.0);
    let _ = evaluate(root.clone(), game, network);
    add_exploration_noise(root.clone(), config);

    for _ in 0..config.number_of_simulations {
        let mut node_ref = root.clone();
        let mut simulation_game = game.clone();
        let mut search_path = vec![node_ref.clone()];

        while node_ref.deref().borrow().expanded() {
            let (action, child) = select_child(node_ref, config);
            simulation_game.apply(action);
            search_path.push(child.clone());
            node_ref = child.clone();
        }

        let value = evaluate(node_ref, &simulation_game, network);
        backpropagate(search_path, value, simulation_game.to_play());
    }

    (select_action::<G>(root.clone(), game, config), root)
}

fn evaluate<G: Game>(node_ref: NodeRef<G>, game: &G, network: &Network<G>) -> f64 {
    let (value, policy_logic_units): (f64, HashMap<G::Action, f64>) =
        network.inference(game.make_image(None));

    let mut node = (*node_ref).borrow_mut();
    node.to_play = game.to_play();
    let policy: HashMap<G::Action, f64> = game
        .legal_actions()
        .into_iter()
        .filter_map(|a| policy_logic_units.get(&a).map(|v| (a, *v)))
        .collect();
    let policy_sum = policy.iter().fold(0.0, |acc, (_, value)| acc + *value);

    for (action, p) in policy.into_iter() {
        node.children.insert(action, Node::new(p / policy_sum));
    }

    value
}

fn add_exploration_noise<G: Game>(node_ref: NodeRef<G>, config: Config) {
    let mut node = (*node_ref).borrow_mut();
    let frac = config.exploration_fraction;
    let gamma = Gamma::new(config.dirichlet_alpha, 1.0).expect("The Config has a bad alpha");
    let gamma = || gamma.sample(&mut rand::thread_rng());
    for (_, mut child) in node.children.iter_mut() {
        let prior = &mut (*child).borrow_mut().prior;
        *prior = *prior * (1.0 - frac) + gamma() * frac;
    }
}

fn select_child<G: Game>(node_ref: NodeRef<G>, config: Config) -> (G::Action, NodeRef<G>) {
    let parent_ref = node_ref.clone();
    let parent = (*node_ref).borrow();
    let mut children = parent.children.iter();
    let (first_action, first_ref) = children.next().unwrap();
    let mut best = (
        ucb_score(&parent, first_ref.clone(), config),
        first_action,
        first_ref,
    );
    for (action, child) in children {
        let child_ucb = ucb_score(&parent, child.clone(), config);
        if child_ucb > best.0 {
            best = (child_ucb, action, child)
        }
    }
    return (*best.1, best.2.clone());
}

fn ucb_score<G: Game>(parent: &Ref<Node<G>>, child_ref: NodeRef<G>, config: Config) -> f64 {
    let child = (*child_ref).borrow();
    let c = ((1 + parent.visit_count + config.c_base) as f64 / config.c_base as f64).ln()
        + config.c_init;
    let u =
        c * parent.prior * (parent.visit_count as f64).sqrt() / (1.0 + child.visit_count as f64);
    u + child.value()
}

fn backpropagate<G: Game>(path: Vec<NodeRef<G>>, value: f64, perspective: Player) {
    for node_ref in path {
        let mut node = (*node_ref).borrow_mut();
        node.summed_value += if node.to_play == perspective {
            value
        } else {
            1.0 - value
        };
        node.visit_count += 1;
    }
}

fn select_action<G: Game>(node: NodeRef<G>, game: &G, config: Config) -> Option<G::Action> {
    if game.len() < config.move_samples {
        None
    } else {
        node.deref()
            .borrow()
            .children
            .iter()
            .reduce(|(_best_action, best), (_child_action, child)| {
                if child.deref().borrow().visit_count > best.deref().borrow().visit_count {
                    (_child_action, child)
                } else {
                    (_best_action, best)
                }
            })
            .map(|(action, _)| *action)
    }
}
