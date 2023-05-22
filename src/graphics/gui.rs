

pub struct Context {
    pub imgui_ctx: imgui::Context,
}

impl Context {
    pub fn new() -> Context {
        let mut imgui = imgui::Context::create();
        Context {
            imgui_ctx: imgui,
        }
    }
}
