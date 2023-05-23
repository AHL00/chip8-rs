use imgui_sdl2::ImguiSdl2;
use sdl2::video::Window;

use super::Graphics;

pub struct Gui {
    pub imgui_ctx: imgui::Context,
    pub imgui_renderer: imgui_opengl_renderer::Renderer,
    pub imgui_sdl2: ImguiSdl2,
}

impl Gui {
    pub fn new(window: &Window) -> Gui {
        let mut imgui_ctx = imgui::Context::create();
        imgui_ctx.set_ini_filename(None);

        let imgui_sdl2 = ImguiSdl2::new(&mut imgui_ctx, &window);

        let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_ctx, |s| {
            window.subsystem().gl_get_proc_address(s) as _
        });

        Gui {
            imgui_ctx,
            imgui_renderer,
            imgui_sdl2,
        }
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event) {
        self.imgui_sdl2.handle_event(&mut self.imgui_ctx, event);
    }

    pub fn render<F>(&mut self, gfx: &Graphics, mut closure: F)
    where
        F: FnMut(&imgui::Ui) -> bool,
    {
        self.imgui_sdl2.prepare_frame(self.imgui_ctx.io_mut(), &gfx.window, &gfx.event_pump.mouse_state());
        
        let frame = self.imgui_ctx.frame();

        let show = closure(&frame);

        if show {
            self.imgui_sdl2.prepare_render(&frame, &gfx.window);
            self.imgui_renderer.render(&mut self.imgui_ctx);
        }
    }
}
