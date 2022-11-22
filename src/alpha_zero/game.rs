pub trait Game: Clone {
    type Action: Send + 'static;
    type Representation;
    fn create(history: Option<Vec<Self::Action>>) -> Self;
    fn terminal(&self) -> bool;
    fn terminal_value(&self) -> f64;
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn apply(&mut self, action: Self::Action);
    fn make_target(&self) -> Self::Representation;
    fn to_play(&self) -> Player;
}

pub enum Player {
    A,
    B
}