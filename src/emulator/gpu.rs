use std::{rc::Rc, cell::RefCell};

pub struct GPU {
    memory_ref: Rc<RefCell<[u8; 4096]>>,
}

impl GPU {
    pub fn new(memory_ref: Rc<RefCell<[u8; 4096]>>) -> GPU {
        GPU {
            memory_ref,
        }
    }
}
