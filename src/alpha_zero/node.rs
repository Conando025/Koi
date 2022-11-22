use super::{game::Player, Game};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Node<G: Game> {
    pub(super) visit_count: usize,
    pub(super) to_play: Player,
    pub(super) prior: f64,
    pub(super) summed_value: f64,
    pub(super) children: HashMap<G::Action, NodeRef<G>>,
}

pub type NodeRef<G> = Rc<RefCell<Node<G>>>;

impl<G: Game> Node<G> {
    pub fn new(prior: f64) -> NodeRef<G> {
        Rc::new(RefCell::new(Node {
            visit_count: 0,
            to_play: Player::A,
            prior,
            summed_value: 0.0,
            children: HashMap::new(),
        }))
    }

    pub fn expanded(&self) -> bool {
        self.children.len() > 0
    }

    pub fn value(&self) -> f64 {
        if self.visit_count == 0 {
            0.0
        } else {
            self.summed_value / (self.visit_count as f64)
        }
    }
}
