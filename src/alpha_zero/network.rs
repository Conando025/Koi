use std::collections::HashMap;
use super::{Config, Game};

#[derive(Clone)]
pub struct Network {}

impl Network {
    pub fn train(&mut self, config: Config) {
        unimplemented!()
    }

    pub fn inference<G: Game>(&self, image: G::Image) -> (f64, HashMap<G::Action, f64>) {
        unimplemented!()
    }
}
