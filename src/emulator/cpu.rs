use fixedstep::FixedStep;
use notan::random::rand;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::DisplayBuffer;

pub struct CPU {
    pub idx: u16,
    pub pc: u16,
    pub cir: u16,
    pub stack: Vec<u8>,
    pub snd_timer: Arc<Mutex<u8>>,
    pub dly_timer: Arc<Mutex<u8>>,
    pub v_reg: [u8; 16],
    pub memory_ref: Rc<RefCell<[u8; 4096]>>,
    pub display_buffer_ref: Arc<Mutex<DisplayBuffer>>,
}

impl CPU {
    pub fn new(
        memory_ref: Rc<RefCell<[u8; 4096]>>,
        display_buffer_ref: Arc<Mutex<DisplayBuffer>>,
    ) -> CPU {
        let mut cpu = CPU {
            idx: 0,
            pc: 0,
            cir: 0,
            stack: Vec::new(),
            dly_timer: Arc::new(Mutex::new(0)),
            snd_timer: Arc::new(Mutex::new(0)),
            v_reg: [0; 16],
            memory_ref,
            display_buffer_ref,
        };

        cpu.start_timer_thread();

        cpu
    }

    fn start_timer_thread(&mut self) {
        let snd_timer = self.snd_timer.clone();
        let dly_timer = self.dly_timer.clone();

        std::thread::spawn(move || {
            // Specification requires 60hz
            let mut fixed_step = FixedStep::start(60.0).unlimit();

            loop {
                // Decrement both timers
                if fixed_step.update() {
                    let mut snd_timer = snd_timer.lock().unwrap();
                    let mut dly_timer = dly_timer.lock().unwrap();

                    if *snd_timer > 0 {
                        *snd_timer -= 1;
                    }

                    if *dly_timer > 0 {
                        *dly_timer -= 1;
                    }
                }
            }
        });
    }

    pub fn fetch(&mut self) {
        let memory = self.memory_ref.borrow();

        // get two consecutive bytes from memory and join them into a single u16
        self.cir = (memory[self.pc as usize] as u16) << 8 | (memory[(self.pc + 1) as usize] as u16);

        // increment the program counter
        self.pc += 2;
    }

    //TODO: Implement better printing to gui
    pub fn decode_execute(&mut self) {
        let instr = Instruction::parse_u16(self.cir);

        if self.cir == 0 {
            // no-op
            println!("--");
            return;
        }

        match instr.opcode & 0xF000 {
            0x0000 => {
                match instr.opcode & 0x00FF {
                    0x00E0 => {
                        // clear screen
                        println!("Clearing screen");
                        self.display_buffer_ref.lock().unwrap().clear(0);
                        self.display_buffer_ref.lock().unwrap().clear(1);
                    }
                    _ => {
                        println!("UNKOWN INSTRUCTION: {:X}", instr.opcode);
                    }
                }
            }
            0x1000 => {
                // jump to address NNN
                println!("Jumping to address {:X}", instr.nnn);
                self.pc = instr.nnn;
            }
            0x2000 => {
                // subroutine at NNN
            }
            0x3000 => {
                // skip next instruction if VX == NN
                println!("Skipping instruction if V{:X} == {:X}", instr.x, instr.nn);
                if self.v_reg[instr.x as usize] == instr.nn {
                    println!("Skipping instruction");
                    self.pc += 2;
                }
            }
            0x4000 => {
                // skip next instruction if VX != NN
                println!("Skipping instruction if V{:X} != {:X}", instr.x, instr.nn);
                if self.v_reg[instr.x as usize] != instr.nn {
                    println!("Skipping instruction");
                    self.pc += 2;
                }
            }
            0x5000 => {
                // skip if registers are equal
                if self.v_reg[instr.x as usize] == self.v_reg[instr.y as usize] {
                    println!("Skipping instruction");
                    self.pc += 2;
                }
            }
            0x6000 => {
                // set VX to NN
                println!("Setting V{:X} to {:X}", instr.x, instr.nn);
                self.v_reg[instr.x as usize] = instr.nn;
            }
            0x7000 => {
                // add values
                println!("Adding {:X} to V{:X}", instr.nn, instr.x);
                let sum = self.v_reg[instr.x as usize] as u16 + instr.nn as u16;
                if sum <= 255 {
                    self.v_reg[instr.x as usize] += instr.nn;
                } else {
                    self.v_reg[instr.x as usize] = 255;
                }
            }
            0x8000 => {
                match instr.n {
                    0x0 => {
                        // store vy in vx
                        println!("Storing V{:X} in V{:X}", instr.y, instr.x);
                        self.v_reg[instr.x as usize] = self.v_reg[instr.y as usize];
                    }
                    0x1 => {
                        // set vx to vx | vy
                        println!("Setting V{:X} to V{:X} | V{:X}", instr.x, instr.x, instr.y);
                        self.v_reg[instr.x as usize] |= self.v_reg[instr.y as usize];
                    }
                    0x2 => {
                        // set vx to vx & vy
                        println!("Setting V{:X} to V{:X} & V{:X}", instr.x, instr.x, instr.y);
                        self.v_reg[instr.x as usize] &= self.v_reg[instr.y as usize];
                    }
                    0x3 => {
                        // set vx to vx ^ vy
                        println!("Setting V{:X} to V{:X} ^ V{:X}", instr.x, instr.x, instr.y);
                        self.v_reg[instr.x as usize] ^= self.v_reg[instr.y as usize];
                    }
                    0x4 => {
                        // add vy to vx, vf = 1 if carry
                        println!(
                            "Adding V{:X} to V{:X} with VF 0x1 if carry",
                            instr.y, instr.x
                        );

                        let sum =
                            (self.v_reg[instr.x as usize] as u16 + self.v_reg[instr.y as usize] as u16);

                        if sum > 0xFF {
                            self.v_reg[0xF] = 1; // Carry
                            self.v_reg[instr.x as usize] = 255;
                        } else {
                            self.v_reg[0xF] = 0; // No carry
                            self.v_reg[instr.x as usize] = sum as u8;
                        }
                    }
                    0x5 => {
                        // subtract vy from vx, vf = 0 if borrow
                        println!(
                            "Subtracting V{:X} to V{:X} with VF 0x0 if carry",
                            instr.y, instr.x
                        );
                        
                        if self.v_reg[instr.x as usize] > self.v_reg[instr.y as usize] {
                            self.v_reg[0xF] = 1; // No borrow
                            self.v_reg[instr.x as usize] -= self.v_reg[instr.y as usize];
                        } else {
                            self.v_reg[0xF] = 0; // Borrow
                            self.v_reg[instr.x as usize] = 0;
                        }
                    }
                    0x6 => {
                        // right shift vy, store in vx, set vf to least significant bit
                        println!(
                            "Right shifting V{:X} and storing in V{:X}, least sig in VF",
                            instr.y, instr.x
                        );
                        let least_sig = self.v_reg[instr.y as usize] & 0x1;
                        self.v_reg[instr.x as usize] = self.v_reg[instr.y as usize] >> 1;
                        self.v_reg[0xF] = least_sig;
                    }
                    0x7 => {
                        // set vx to vy - vx, vf = 0 if borrow
                        println!(
                            "Subtracting V{:X} to V{:X} with VF 0x0 if carry",
                            instr.x, instr.y
                        );

                        self.v_reg[instr.x as usize] =
                            self.v_reg[instr.y as usize] - self.v_reg[instr.x as usize];

                        if self.v_reg[instr.y as usize] > self.v_reg[instr.x as usize] {
                            self.v_reg[0xF] = 1; // No borrow
                        } else {
                            self.v_reg[0xF] = 0; // Borrow
                        }
                    }
                    0xE => {
                        // right shift vy, store in vx, set vf to most significant bit
                        println!(
                            "Left shifting V{:X} and storing in V{:X}, most sig in VF",
                            instr.y, instr.x
                        );
                        let most_sig = self.v_reg[instr.y as usize] & 0x80;
                        self.v_reg[instr.x as usize] = self.v_reg[instr.y as usize] << 1;
                        self.v_reg[0xF] = most_sig;
                    }
                    _ => {
                        // TODO: panic
                    }
                }
            }
            0x9000 => {
                // if vy != vx, skip next instruction
                println!(
                    "Skipping next instruction if V{:X} != V{:X}",
                    instr.x, instr.y
                );
                if self.v_reg[instr.x as usize] != self.v_reg[instr.y as usize] {
                    println!("Skipping next instruction");
                    self.pc += 2;
                }
            }
            0xA000 => {
                // store NNN in index reg
                println!("Storing {:X} in index reg", instr.nnn);
                self.idx = instr.nnn;
            }
            0xF000 => {
                match instr.nn {
                    0x1E => {
                        // add vx to index reg
                        println!("Adding V{:X} to index reg", instr.x);
                        self.idx += self.v_reg[instr.x as usize] as u16;
                    }
                    0x65 => {
                        // load registers V0 through VX from memory starting at location I
                        println!("Loading registers V0 through V{:X} from memory", instr.x);
                        for i in 0..instr.x + 1 {
                            self.v_reg[i as usize] =
                                self.memory_ref.borrow()[(self.idx + i as u16) as usize];
                        }
                    }
                    _ => {
                        println!("UNKOWN INSTRUCTION: {:X}", instr.opcode);
                    }
                }
                // Add the value stored in register VX to register I
                println!("Adding V{:X} to index reg", instr.x);
                self.idx += self.v_reg[instr.x as usize] as u16;
            }
            0xD000 => {
                // draw pixel
                println!(
                    "Drawing sprite at ({:X}, {:X}) with height {:X}",
                    self.v_reg[instr.x as usize], self.v_reg[instr.y as usize], instr.n
                );

                // reset VF
                self.v_reg[0xF] = 0;

                // get buffer
                let mut buffer = self
                    .display_buffer_ref
                    .lock()
                    .unwrap()
                    .current_buffer()
                    .clone();

                // get coords
                let mut coords: (u8, u8) =
                    (self.v_reg[instr.x as usize], self.v_reg[instr.y as usize]);

                // if coords are over the screen, wrap them
                coords.0 = coords.0 % 64;
                coords.1 = 31 - (coords.1 % 32); // flip coords

                for i in 0..instr.n {
                    let sprite_row = self.memory_ref.borrow()[(self.idx + i as u16) as usize];

                    for j in 0..8 {
                        let sprite_pixel = (sprite_row >> (7 - j)) & 0x1;

                        let b_idx_1 = (coords.1 - i) as usize; // y
                        let b_idx_2 = (coords.0 + j) as usize; // x

                        if b_idx_1 >= 32 {
                            continue;
                        } // y
                        if b_idx_2 >= 64 {
                            continue;
                        } //x

                        let pixel = buffer[b_idx_1][b_idx_2];

                        if sprite_pixel == 1 {
                            if pixel != 0 {
                                self.v_reg[0xF] = 1;
                                buffer[b_idx_1][b_idx_2] = 0;
                            } else {
                                buffer[b_idx_1][b_idx_2] = 255;
                            }
                        }
                    }
                }

                println!("VF: {}", self.v_reg[0xF]);

                self.display_buffer_ref
                    .lock()
                    .unwrap()
                    .set_buffer(-1, buffer);
            }
            0xC000 => {
                // set VX to rand() & NN
                self.v_reg[instr.x as usize] = rand::random::<u8>() & instr.nn;
                println!(
                    "Random V{:X} to {:X}",
                    instr.x, self.v_reg[instr.x as usize]
                );
            }
            _ => {
                //TODO: panic!("Unknown opcode: {:X}", instr.opcode);
                println!("UNKOWN INSTRUCTION: {:X}", instr.opcode);
            }
        }
    }
}

pub struct Instruction {
    opcode: u16,
    nnn: u16,
    nn: u8,
    n: u8,
    x: u8,
    y: u8,
}

impl Instruction {
    pub fn parse_u16(opcode: u16) -> Instruction {
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        Instruction {
            opcode,
            nnn,
            nn,
            n,
            x,
            y,
        }
    }
}
