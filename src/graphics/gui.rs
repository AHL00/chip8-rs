use imgui_sdl2::ImguiSdl2;
use sdl2::video::Window;

pub struct Gui {
    pub imgui_ctx: imgui::Context,
    pub imgui_renderer: imgui_opengl_renderer::Renderer,
    pub imgui_sdl2: ImguiSdl2,
}

impl Gui {
    pub fn new(window: &Window) -> Gui {
        let mut imgui_ctx = imgui::Context::create();
        imgui_ctx.set_ini_filename(None);

        let mut imgui_sdl2 = ImguiSdl2::new(&mut imgui_ctx, &window);

        let mut imgui_renderer =
            imgui_opengl_renderer::Renderer::new(&mut imgui_ctx, |s| {
                window.subsystem().gl_get_proc_address(s) as _
            });

        Gui {
            imgui_ctx,
            imgui_renderer,
            imgui_sdl2
        }
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event) {
        self.imgui_sdl2.handle_event(&mut self.imgui_ctx, event);
    }

    pub fn render(&mut self, window: &Window, mouse_state: &sdl2::mouse::MouseState) {
        self.imgui_sdl2.prepare_frame(self.imgui_ctx.io_mut(), &window, mouse_state);
        let frame = self.imgui_ctx.new_frame();

        frame.show_demo_window(&mut true);

        unsafe {
            gl::ClearColor(1.0, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.imgui_sdl2.prepare_render(&frame, &window);
        self.imgui_renderer.render(&mut self.imgui_ctx);
    }
}