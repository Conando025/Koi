use super::{Config, Game};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Network<G: Game> {
    _game: PhantomData<G>,
}

impl<G: Game> Network<G> {
    pub fn new() -> Network<G> {
        Self {
            _game: PhantomData::default(),
        }
    }

    pub fn train(&mut self, config: Config) {
        unimplemented!()
    }

    pub fn inference(&self, image: G::Image) -> (f64, HashMap<G::Action, f64>) {
        unimplemented!()
    }
}
