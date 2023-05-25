use crate::{DisplayBuffer, DebugInfo};
use fixedstep::FixedStep;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex}, fmt::Debug,
};

pub struct GPU {
    memory_ref: Rc<RefCell<[u8; 4096]>>,
    local_display_buffer: Arc<Mutex<[[u8; 64]; 32]>>,
    display_buffer: Arc<Mutex<DisplayBuffer>>,
    debug_info: Arc<Mutex<DebugInfo>>,
}

impl GPU {
    pub fn new(
        memory_ref: Rc<RefCell<[u8; 4096]>>,
        refresh_rate: f32,
        display_buffer: Arc<Mutex<DisplayBuffer>>,
        debug_info: Arc<Mutex<DebugInfo>>,
    ) -> GPU {
        let gpu = GPU {
            memory_ref,
            local_display_buffer: Arc::new(Mutex::new([[0; 64]; 32])),
            display_buffer,
            debug_info,
        };

        gpu.display_refresh_thread(
            refresh_rate,
            gpu.local_display_buffer.clone(),
            gpu.display_buffer.clone(),
        );

        gpu
    }

    /// Separate thread for refreshing the display
    pub fn display_refresh_thread(
        &self,
        refresh_rate: f32,
        local_display_buffer: Arc<Mutex<[[u8; 64]; 32]>>,
        display_buffer: Arc<Mutex<DisplayBuffer>>,
    ) {
        let mut timer = FixedStep::start(refresh_rate as f64).unlimit();
        let display_buffer = display_buffer.clone();
        let local_display_buffer = local_display_buffer.clone();
        let debug_info = self.debug_info.clone();

        let mut last_frame = std::time::Instant::now();

        //TODO: error detection?
        std::thread::spawn(move || {
            loop {
                if timer.update() {
                    let frame_time = last_frame.elapsed().as_secs_f32() * 1000.0;
                    debug_info.lock().unwrap().frame_time = frame_time;
                    last_frame = std::time::Instant::now();

                    let mut display_buffer = display_buffer.lock().unwrap();

                    // set inactive buffer to local buffer
                    display_buffer
                        .set_buffer(-2, *local_display_buffer.lock().unwrap());

                    // swap buffers
                    display_buffer.swap_buffers();
                }
            }
        });
    }

    pub fn clear_screen(&mut self) {
        *self.local_display_buffer.lock().unwrap() = [[0; 64]; 32];
    }

    /// Draws sprite at coords, returns true if any pixels were erased
    pub fn draw(&mut self, mut coords: (u8, u8), sprite: &[u8]) -> bool {
        // if coords are over the screen, wrap them
        coords.0 = coords.0 % 64;
        coords.1 = 31 - (coords.1 % 32); // flip y axis

        let mut erased = false;

        for y in 0..sprite.len() {
            let mut sprite_byte = sprite[y];

            for x in 0..8 {
                let sprite_pixel = (sprite_byte >> (7 - x)) & 0x1;

                let x = (coords.0 + x) % 64;
                let y = (coords.1 - y as u8) % 32;

                if x >= 64 {
                    continue;
                }

                if y >= 32 {
                    continue;
                }

                let mut display_buffer = self.local_display_buffer.lock().unwrap();

                let pixel = display_buffer[y as usize][x as usize];

                if sprite_pixel == 1 {
                    if pixel != 0 {
                        erased = true;
                        display_buffer[y as usize][x as usize] = 0;
                    } else {
                        display_buffer[y as usize][x as usize] = 255;
                    }
                }
            }
        }

        erased
    }
}
