use std::sync::{Mutex, Arc};

use notan::egui::{self, *};
use notan::prelude::*;

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

    pub fn set_buffer(&mut self, index: usize, buffer: [[u8; 64]; 32]) {
        self.buffer[index] = buffer;
    }

    pub fn clear(&mut self, index: usize) {
        self.buffer[index] = [[0; 64]; 32];
    }
}

pub struct DebugInfo {
    pub ips: u64,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            ips: 0,
        }
    }
}

#[derive(AppState)]
pub struct State {
    pub emulator_out_texture: RenderTexture,
    pub emulator_out_tex_id: egui::TextureId,
    pub display_buffer: Arc<Mutex<DisplayBuffer>>,
    pub debug_info: Arc<Mutex<DebugInfo>>,
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
        }
    }
}