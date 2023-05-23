#[macro_use]
extern crate imgui;
extern crate gl;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

use chip8_rs::graphics::{self, gui};

fn main() {
    let mut gfx_ctx = graphics::Graphics::new();

    let mut gui_ctx = gui::Gui::new(&gfx_ctx.window);

    'running: loop {
        for event in gfx_ctx.event_pump.poll_iter() {
            gui_ctx.handle_event(&event);

            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => { },
            }
        }

        gfx_ctx.render();

        gui_ctx.render(&gfx_ctx, |ui| {
            ui.text("Hello world!");
            ui.show_demo_window(&mut true);

            true
        });

        gfx_ctx.swap_window();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}