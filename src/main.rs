mod app;
mod window;
mod imgui_instance;
mod imgui_impl_glfw;

extern crate glfw;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate glow;

use crate::app::{ AppBuilder };

fn main() {
    let mut app = AppBuilder::new().window().build();
    app.run();
}