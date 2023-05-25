use std::process::exit;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use chip8_rs::DebugInfo;
use chip8_rs::emulator;
use chip8_rs::graphics;
use chip8_rs::State;
use fixedstep::FixedStep;
use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(
            WindowConfig::new()
                .vsync(true)
                .high_dpi(true)
                .multisampling(4)
                .title("Chip8 Emulator")
                .resizable(true),
        )
        .add_config(EguiConfig)
        .add_config(DrawConfig)
        .draw(graphics::render)
        .initialize(start)
        .build()
}

fn start(state: &mut State) {
    let display_buffer = state.display_buffer.clone();
    let debug_info = state.debug_info.clone();

    // fill display buffer with test data, random data. 0 or 255
    // for row in display_buffer.lock().unwrap().inactive_buffer_mut().iter_mut() {
    //     for col in row.iter_mut() {
    //         if rand::random::<bool>() {
    //             *col = 255;
    //         } else {
    //             *col = 0;
    //         }
    //     }
    // }

    // display_buffer.lock().unwrap().swap_buffers();

    let emu_thread = std::thread::spawn(move || {
        let mut emu = emulator::Emulator::new(display_buffer.clone(), debug_info.clone());

        // Initialize the emulator
        emu.initialize("roms/ibm.ch8", 10);
        //emu.initialize("roms/test_opcode.ch8", 1000000000);
        //emu.initialize("roms/ibm.ch8", 3000);

        // ask user input in console
        // println!("Clock speed? (Hz): ");
        // let mut input = String::new();
        // let clock_input = std::io::stdin().read_line(&mut input).unwrap();
        // let clock_speed = input.trim().parse::<u32>().unwrap();

        // println!("Rom path?: ");
        // input = String::new();
        // let rom_input = std::io::stdin().read_line(&mut input).unwrap();
        // let rom_path = input.trim();

        // emu.initialize(rom_path, clock_speed);

        let mut loops_last_second = 0;
        let mut last_second = std::time::Instant::now();

        let mut fixed_step = FixedStep::start(emu.clock_speed as f64).unlimit(); // 60.0Hz

        loop {
            if fixed_step.update() {
                if last_second.elapsed().as_secs_f64() < 1.0 {
                    loops_last_second += 1;
                } else {
                    debug_info.lock().unwrap().clock_speed = loops_last_second;
                    loops_last_second = 0;
                    last_second = std::time::Instant::now();
                }

                emu.cycle();
            }
        }
    });
    
    state.emu_thread_handle = Some(emu_thread);
}
