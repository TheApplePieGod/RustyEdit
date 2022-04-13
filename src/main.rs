mod app;
mod app_state;
mod window;
mod imgui_instance;
mod imgui_impl_glfw;
mod viewport;
mod texture;
mod asset_manager;
mod logging;
mod widget_manager;
mod widget_state;
mod image_utils;
mod color;

extern crate glfw;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate gl;
extern crate image;
extern crate log;
extern crate termcolor;
extern crate chrono;

use crate::{app::{ App }, logging::ConsoleLogger};

static LOGGER: ConsoleLogger = ConsoleLogger;

fn main() {
    // Initialize logging
    match log::set_logger(&LOGGER) {
        Ok(_) => log::set_max_level(log::LevelFilter::Trace),
        Err(e) => println!("Failed to initialize logger: {}", e)
    }

    // Run the app
    let mut app = App::new();
    app.run();
}