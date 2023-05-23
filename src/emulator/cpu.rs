use std::{cell::RefCell, rc::Rc};



pub struct CPU {
    index_reg: u16,
    pc_reg: u16,
    stack: Vec<u8>,
    dly_timer: u8,
    snd_timer: u8,
    v_reg: [u8; 16], 
    memory_ref: Rc<RefCell<[u8; 4096]>>,
}

impl CPU {
    pub fn new(memory_ref: Rc<RefCell<[u8; 4096]>>) -> CPU {
        CPU {
            index_reg: 0,
            pc_reg: 0,
            stack: Vec::new(),
            dly_timer: 0,
            snd_timer: 0,
            v_reg: [0; 16],
            memory_ref: memory_ref,
        }
    }

    pub fn fetch(&mut self) {

    }

    pub fn decode(&mut self) {

    }

    pub fn execute(&mut self) {

    }
}