use super::{Config, Game, Network, Node, NodeRef};
use std::collections::HashMap;

pub fn run<G: Game>(config: Config, game: &G, network: &Network) -> (G::Action, Node<G>) {
    let mut root = Node::new(0.0);
    let _ = evaluate(root, game, network);
    unimplemented!()
}

fn evaluate<G: Game>(node: NodeRef<G>, game: &G, network: &Network) -> f64 {
    let (value, policy_logic_units): (f64, HashMap<G::Action, f64>) =
        network.inference::<G>(game.make_image(None));

    node.to_play = game.to_play();
    let policy: HashMap<G::Action, f64> = game
        .legal_actions()
        .into_iter()
        .filter_map(|a| policy_logic_units.get(&a).map(|v| (a, *v)))
        .collect();
    let policy_sum = policy.iter().fold(0.0, |acc, (_, value)| acc + *value);

    for (action, p) in policy.into_iter() {
        node.children.insert(action, Node::new(p / policy_sum))
    }

    value
}
