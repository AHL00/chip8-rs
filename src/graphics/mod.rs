use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;

use super::State;

pub mod gui;

pub fn render(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    // read buffer
    let buffer = state
        .display_buffer
        .lock()
        .unwrap()
        .current_buffer()
        .clone();

    let mut draw = state.emulator_out_texture.create_draw();

    draw.clear(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    });

    // draw the buffer to the render texture
    for (i, row) in buffer.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 255 {
                draw.rect((j as f32, i as f32), (1.0, 1.0));
            }
        }
    }

    gfx.render_to(&state.emulator_out_texture, &draw);

    // create an egui output
    let mut output = plugins.egui(|ctx| {
        egui::Window::new("")
            .title_bar(false)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .show(ctx, |ui| {
                // draw the render texture to egui
                let size: egui::Vec2 = state.emulator_out_texture.size().into();
                ui.image(state.emulator_out_tex_id, (512.0, 256.0));
            });

        egui::Window::new("Debug")
            .vscroll(false)
            .hscroll(false)
            .collapsible(true)
            .default_width(100.0)
            .anchor(Align2::LEFT_TOP, (0.0, 0.0))
            .resizable(false)
            .show(ctx, |ui| {
                ui.add(Label::new(format!("IPS: {}", state.debug_info.lock().unwrap().ips)));
            });
    });

    output.clear_color(Color::BLACK);
    gfx.render(&output);
}

fn test_buffer() -> [[u8; 64]; 32] {
    // alternating colors
    let mut buffer = [[0; 64]; 32];
    for (i, row) in buffer.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            if (i + j) % 2 == 0 {
                *col = 255;
            }
        }
    }

    buffer
}
