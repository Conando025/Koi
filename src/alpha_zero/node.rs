use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    visit_count: usize,
    to_play: f64,
    prior: f64,
    summed_value: f64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn empty(prior: f64) -> Self {
        Node {
            visit_count: 0,
            to_play: -1.0,
            prior,
            summed_value: 0.0,
            children: Vec::new(),
        }
    }

    fn expanded(&self) -> bool {
        self.children.len() > 0
    }

    fn value(&self) -> f64 {
        if self.visit_count == 0 {
            0.0
        } else {
            self.summed_value / (self.visit_count as f64)
        }

    }
}