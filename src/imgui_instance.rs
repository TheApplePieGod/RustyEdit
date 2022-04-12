use crate::{imgui_impl_glfw::ImGuiGLFW};

pub struct ImGuiInstance {
    glfw: ImGuiGLFW,
}

impl ImGuiInstance {
    pub fn new(window: &mut glfw::Window, context: &mut imgui::Context) -> Self {
        Self {
            glfw: ImGuiGLFW::new(context, window),
        }
    }

    pub fn handle_event(&mut self, context: &mut imgui::Context, event: &glfw::WindowEvent) {
        self.glfw.handle_event(context, event);
    }

    pub fn begin_frame<'a>(&mut self, window: &mut glfw::Window, context: &'a mut imgui::Context) -> imgui::Ui<'a> {
        self.glfw.frame(window, context)
    }

    pub fn end_frame(&mut self, window: &mut glfw::Window, ui: imgui::Ui) {
        self.glfw.draw(ui, window);
    }
}