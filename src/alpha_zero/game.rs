use std::hash::Hash;
use super::Node;

pub trait Game: Clone + Send {
    type Action: Hash + Eq;
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
    fn store_search_statistics(&mut self, root: Node<Self>);
    fn make_image(&self, index: Option<usize>) -> Self::Image;
}

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
