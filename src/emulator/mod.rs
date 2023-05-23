pub mod gpu;
use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use gpu::GPU;

pub mod cpu;
use cpu::CPU;

use crate::DisplayBuffer;

pub struct Emulator {
    cpu: CPU,
    memory: Rc<RefCell<[u8; 4096]>>,
    gpu: GPU,
    display_buffer: Arc<Mutex<DisplayBuffer>>,
    pub clock_speed: u32,
}

impl Emulator {
    pub fn new(display_buffer: Arc<Mutex<DisplayBuffer>>) -> Emulator {
        let memory = Rc::new(RefCell::new([0; 4096]));
        let cpu = CPU::new(memory.clone(), display_buffer.clone());
        let gpu = GPU::new(memory.clone());
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
            display_buffer,
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
        self.cpu.pc = 0x200;
        self.load_rom_from_file(rom_path);
        self.clock_speed = clock_speed;
    }

    pub fn cycle(&mut self) {
        self.cpu.fetch();
        self.cpu.decode_execute();
    }
}