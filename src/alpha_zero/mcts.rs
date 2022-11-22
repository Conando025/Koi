use super::{Config, Game, Network, Node};

pub fn run<G: Game>(config: Config, game: &G, network: &Network) -> (G::Action, Node) {
    let mut root = Node::empty(0.0);
    evaluate(&mut root, game, network);
    unimplemented!()
}

fn evaluate<G: Game>(root: &mut Node, game: &G, network: &Network) {
    unimplemented!()
}