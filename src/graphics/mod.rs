use imgui_sdl2::ImguiSdl2;
use sdl2::{video::Window, EventPump, Sdl, VideoSubsystem};
extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

pub mod gui;

pub struct Graphics {
    pub sdl_ctx: Sdl,
    pub video: VideoSubsystem,
    pub window: Window,
    pub event_pump: EventPump,
    // needed so glcontext isnt disposed at end of function
    _gl_context: sdl2::video::GLContext,
}

impl Graphics {
    pub fn new() -> Graphics {
        let sdl_ctx = sdl2::init().unwrap();
        let video = sdl_ctx.video().unwrap();

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


        let event_pump = sdl_ctx.event_pump().unwrap();

        Graphics {
            sdl_ctx,
            video,
            window,
            event_pump,
            _gl_context,
        }
    }

    pub fn render(&mut self) {
        // render queue?
    }

    pub fn swap_window(&mut self) {
        self.window.gl_swap_window();
    }
}