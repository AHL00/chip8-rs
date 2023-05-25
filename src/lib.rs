use std::sync::{Mutex, Arc};
use std::time::Instant;
use notan::egui::{self, *};
use notan::prelude::*;
use fixedstep::FixedStep;

pub mod emulator;
pub mod graphics;

pub struct DisplayBuffer {
    buffer: [[[u8; 64]; 32]; 2],
    current_buffer: usize,
}

impl DisplayBuffer {
    pub fn new() -> Self {
        DisplayBuffer {
            buffer: [[[0; 64]; 32]; 2],
            current_buffer: 0,
        }
    }

    pub fn swap_buffers(&mut self) {
        self.current_buffer = 1 - self.current_buffer;
    }
    
    pub fn current_buffer_index(&self) -> usize {
        self.current_buffer
    }

    pub fn current_buffer(&self) -> &[[u8; 64]; 32] {
        &self.buffer[self.current_buffer]
    }

    pub fn current_buffer_mut(&mut self) -> &mut [[u8; 64]; 32] {
        &mut self.buffer[self.current_buffer]
    }

    pub fn inactive_buffer(&self) -> &[[u8; 64]; 32] {
        &self.buffer[1 - self.current_buffer]
    }

    pub fn inactive_buffer_mut(&mut self) -> &mut [[u8; 64]; 32] {
        &mut self.buffer[1 - self.current_buffer]
    }

    /// Set the buffer at the given index, index -1 means current buffer, -2 means inactive buffer
    pub fn set_buffer(&mut self, index: isize, buffer: [[u8; 64]; 32]) {
        if index < 0 {
            self.buffer[self.current_buffer] = buffer;
            return;
        }

        if index < -1 {
            self.buffer[1 - self.current_buffer] = buffer;
            return;
        }

        self.buffer[index as usize] = buffer;
    }

    pub fn clear(&mut self, index: usize) {
        self.buffer[index] = [[0; 64]; 32];
    }
}

pub struct DebugInfo {
    pub clock_speed: u64,
    pub frame_time: f32,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            clock_speed: 0,
            frame_time: 0.0,
        }
    }
}

#[derive(AppState)]
pub struct State {
    pub emulator_out_texture: RenderTexture,
    pub emulator_out_tex_id: egui::TextureId,
    pub display_buffer: Arc<Mutex<DisplayBuffer>>,
    pub debug_info: Arc<Mutex<DebugInfo>>,
    pub emu_thread_handle: Option<std::thread::JoinHandle<isize>>,
    pub last_frame: Instant,
    pub render_timer: FixedStep,
    // TODO: Add UI state to this so it can be edited from anywhere
}

impl State {
    pub fn new(gfx: &mut Graphics) -> State {
        let render_texture = gfx
            .create_render_texture(64, 32)
            .with_depth()
            .build()
            .unwrap();

        let tex_id = gfx.egui_register_texture(&render_texture);

        Self {
            emulator_out_tex_id: tex_id,
            emulator_out_texture: render_texture,
            display_buffer: Arc::new(Mutex::new(DisplayBuffer::new())),
            debug_info: Arc::new(Mutex::new(DebugInfo::new())),
            emu_thread_handle: None,
            last_frame: Instant::now(),
            render_timer: FixedStep::start(60.0).unlimit(),
        }
    }
}