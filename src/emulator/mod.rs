pub mod gpu;
use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use gpu::GPU;

pub mod cpu;
use cpu::CPU;

use crate::{DisplayBuffer, DebugInfo};

pub struct Emulator {
    cpu: Rc<RefCell<CPU>>,
    memory: Rc<RefCell<[u8; 4096]>>,
    gpu: Rc<RefCell<GPU>>,
    debug_info: Arc<Mutex<DebugInfo>>,
    pub clock_speed: u32,
}

impl Emulator {
    pub fn new(display_buffer: Arc<Mutex<DisplayBuffer>>, debug_info: Arc<Mutex<DebugInfo>>) -> Emulator {
        let memory = Rc::new(RefCell::new([0; 4096]));
        let gpu = Rc::new(RefCell::new(GPU::new(memory.clone(), 60.0, display_buffer, debug_info.clone())));
        let cpu = Rc::new(RefCell::new(CPU::new(memory.clone(), gpu.clone())));
        let clock_speed = 10;

        // Font to memory
        let font = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        memory.borrow_mut()[0x000..0x050].copy_from_slice(&font[0..0x050]);

        Emulator {
            cpu,
            memory,
            gpu,
            debug_info,
            clock_speed,
        }
    }

    pub fn load_rom_from_file(&mut self, path: &str) {
        let rom = std::fs::read(path).unwrap();
        println!("Loaded rom: {:?}", path);
        self.load_rom(rom);
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.borrow_mut()[0x200..0x200 + rom.len()].copy_from_slice(&rom);
        //println!("Memory slice at {:#03x} <-> {:#03x}:\n{:?}", 0x200, 0x200 + rom.len(), &self.memory.borrow()[0x200..0x200 + rom.len()]);
    }

    pub fn initialize(&mut self, rom_path: &str, clock_speed: u32) {
        self.cpu.borrow_mut().pc = 0x200;
        self.load_rom_from_file(rom_path);
        self.clock_speed = clock_speed;
    }

    pub fn cycle(&mut self) {
        let mut cpu = self.cpu.borrow_mut();
        cpu.fetch();
        cpu.decode_execute();
    }
}