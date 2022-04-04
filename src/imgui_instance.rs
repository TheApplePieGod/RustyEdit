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

    pub fn render_ui<F: Fn(&imgui::Ui)>(&mut self, window: &mut glfw::Window, context: &mut imgui::Context, render: F) {
        let ui = self.glfw.frame(window, context);
        render(&ui);
        self.glfw.draw(ui, window);
    }
}