use super::Node;

pub trait Game: Clone + Send {
    type Action;
    type Representation;
    fn create(history: Option<Vec<Self::Action>>) -> Self;
    fn terminal(&self) -> bool;
    fn terminal_value(&self) -> f64;
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn apply(&mut self, action: Self::Action);
    fn make_target(&self) -> Self::Representation;
    fn to_play(&self) -> Player;
    fn len(&self) -> usize;
    fn store_search_statistics(&mut self, root: Node);
}

pub enum Player {
    A,
    B
}