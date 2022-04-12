mod app;
mod window;
mod imgui_instance;
mod imgui_impl_glfw;
mod viewport;
mod texture;
mod asset_manager;

extern crate glfw;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate gl;
extern crate image;

use crate::app::{ App };

fn main() {
    let mut app = App::new();
    app.run();
}