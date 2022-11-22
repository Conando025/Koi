use super::{Config, Game, Network, Node, NodeRef};
use rand_distr::{Distribution, Gamma};
use std::collections::HashMap;

pub fn run<G: Game>(config: Config, game: &G, network: &Network<G>) -> (G::Action, Node<G>) {
    let mut root = Node::new(0.0);
    let _ = evaluate(root, game, network);
    unimplemented!()
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
