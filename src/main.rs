mod app;
mod window;
mod imgui_instance;
mod imgui_impl_glfw;
mod viewport;

extern crate glfw;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate gl;

use crate::app::{ App };

fn main() {
    let mut app = App::new();
    app.run();
}