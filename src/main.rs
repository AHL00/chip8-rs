use std::sync::Arc;

use chip8_rs::DebugInfo;
use chip8_rs::emulator;
use chip8_rs::graphics;
use chip8_rs::State;
use fixedstep::FixedStep;
use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;
use notan::random::rand;

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

    std::thread::spawn(move || {
        let mut emu = emulator::Emulator::new();

        let mut test_counter = 0;

        let mut loops_last_second = 0;
        let mut last_second = std::time::Instant::now();

        let mut fixed_step = FixedStep::start(emu.ips as f64 * (4.0 / 3.0)); // 60.0Hz

        loop {
            if fixed_step.update() {
                if last_second.elapsed().as_secs_f64() < 1.0 {
                    loops_last_second += 1;
                } else {
                    debug_info.lock().unwrap().ips = loops_last_second;
                    loops_last_second = 0;
                    last_second = std::time::Instant::now();
                }

                if test_counter < 1 {
                    test_counter += 1;
                } else {
                    test_counter = 0;

                    // test data
                    for (i, row) in display_buffer
                        .lock()
                        .unwrap()
                        .inactive_buffer_mut()
                        .iter_mut()
                        .enumerate()
                    {
                        for (j, col) in row.iter_mut().enumerate() {
                            // random pixel
                            if rand::random::<bool>() {
                                *col = 255;
                            } else {
                                *col = 0;
                            }
                        }
                    }

                    display_buffer.lock().unwrap().swap_buffers();
                }
            }
        }
    });
}
