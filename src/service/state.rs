use std::{cell::RefCell, rc::Rc};
use crate::service::blockchain::Blockchain;

#[derive(Debug, Clone)]
pub struct State {
    pub difficulty: Rc<RefCell<usize>>,
    pub blockchain: Rc<RefCell<Blockchain>>,
}

impl State {
    pub fn new(value: usize) -> Self {
        Self {
            difficulty: Rc::new(RefCell::new(value.clone())),
            blockchain: Rc::new(RefCell::new(Blockchain::new(value.clone()))),
        }
    }
}
