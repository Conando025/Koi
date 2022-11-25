use super::Node;
use crate::alpha_zero::node::NodeRef;
use std::hash::Hash;

pub type Target = (i8, f64);

pub trait Game: Clone + Send {
    type Action: Hash + Eq + Copy;
    type Image;
    fn create(history: Option<Vec<Self::Action>>) -> Self;
    fn terminal(&self) -> bool;
    fn terminal_value(&self) -> i8;
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn apply(&mut self, action: Self::Action);
    fn make_target(&self, index: usize) -> Target;
    fn to_play(&self) -> Player;
    fn len(&self) -> usize;
    fn store_search_statistics(&mut self, root: NodeRef<Self>);
    fn make_image(&self, index: usize) -> Self::Image;
    fn make_uniform_network() -> tensorflow::Graph;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Player {
    A,
    B,
}

impl Player {
    fn inverse(self) -> Self {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }
}
