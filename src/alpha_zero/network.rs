use super::{Config, Game};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions};

pub struct ShareableNetwork<G: Game> {
    _game: PhantomData<G>,
    graph: Graph,
}

impl<G: Game> ShareableNetwork<G> {
    pub fn new() -> ShareableNetwork<G> {
        let g = G::make_uniform_network();
        Self {
            _game: PhantomData::default(),
            graph: g,
        }
    }

    pub fn to_network(&self) -> Network<G> {
        Network {
            _game: PhantomData,
            session: Session::new(&SessionOptions::default(), &self.graph).unwrap(),
        }
    }
}

impl<G: Game> Clone for ShareableNetwork<G> {
    fn clone(&self) -> Self {
        let mut g = Graph::new();
        let graph_def = self.graph.graph_def().expect("Unable to serialize Graph");
        g.import_graph_def(&*graph_def, &ImportGraphDefOptions::default());
        Self {
            _game: PhantomData,
            graph: g,
        }
    }
}

pub struct Network<G: Game> {
    _game: PhantomData<G>,
    session: Session,
}

impl<G: Game> Network<G> {
    pub fn train(&mut self, config: Config) {
        unimplemented!()
    }

    pub fn inference(&self, image: G::Image) -> (f64, HashMap<G::Action, f64>) {
        let mut args = tensorflow::SessionRunArgs::new();
        self.session
            .run(&mut args)
            .expect("Running the network resulted in an error.");
        unimplemented!();
    }
}
