use std::borrow::{Borrow, BorrowMut};

use imgui_sdl2::ImguiSdl2;
use sdl2::{video::Window, EventPump, Sdl, VideoSubsystem};
extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

pub struct Context {
    pub sdl_context: Sdl,
    pub video: VideoSubsystem,
    pub window: Window,
    pub event_pump: EventPump,
    pub imgui_renderer: imgui_opengl_renderer::Renderer,
    pub imgui_sdl2_ctx: ImguiSdl2,
    pub imgui_ctx: imgui::Context,
    // needed so glcontext isnt disposed at end of function
    _gl_context: sdl2::video::GLContext,
}

impl Context {
    pub fn new() -> Context {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();

        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
        }

        let window = video
            .window("rust-imgui-sdl2 demo", 1000, 1000)
            .position_centered()
            .resizable()
            .opengl()
            .allow_highdpi()
            .build()
            .unwrap();

        let _gl_context = window
            .gl_create_context()
            .expect("Couldn't create GL context");
        gl::load_with(|s| video.gl_get_proc_address(s) as _);

        let mut imgui = imgui::Context::create();

        let mut imgui_sdl2 = ImguiSdl2::new(&mut imgui, &window);

        let mut imgui_renderer =
            imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

        let mut event_pump = sdl_context.event_pump().unwrap();

        Context {
            sdl_context,
            video,
            window,
            event_pump,
            imgui_renderer,
            imgui_sdl2_ctx: imgui_sdl2,
            imgui_ctx: imgui,
            _gl_context,
        }
    }

    pub fn render(&mut self) {
        self.imgui_sdl2_ctx.prepare_frame(self.imgui_ctx.io_mut(), &self.window, &self.event_pump.mouse_state());
        let ui = self.imgui_ctx.borrow_mut().frame();

        ui.show_demo_window(&mut true);

        unsafe {
            gl::ClearColor(1.0, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.imgui_sdl2_ctx.prepare_render(&ui, &self.window);
        self.imgui_renderer.render(&mut self.imgui_ctx);

        self.window.gl_swap_window();
    }

    // Put in loop, returns true if main loop should be broken
    pub fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            self.imgui_sdl2_ctx.handle_event(&mut self.imgui_ctx, &event);
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => { println!("Exiting..."); return true },
                _ => {}
            }
        }
        return false;
    }
}