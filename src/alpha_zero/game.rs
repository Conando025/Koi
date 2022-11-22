use super::Node;
use std::hash::Hash;
use crate::alpha_zero::node::NodeRef;

pub trait Game: Clone + Send {
    type Action: Hash + Eq + Copy;
    type Representation;
    type Image;
    fn create(history: Option<Vec<Self::Action>>) -> Self;
    fn terminal(&self) -> bool;
    fn terminal_value(&self) -> f64;
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn apply(&mut self, action: Self::Action);
    fn make_target(&self) -> Self::Representation;
    fn to_play(&self) -> Player;
    fn len(&self) -> usize;
    fn store_search_statistics(&mut self, root: NodeRef<Self>);
    fn make_image(&self, index: Option<usize>) -> Self::Image;
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
