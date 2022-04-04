use std::{sync::mpsc::Receiver};
use glfw::{Action, Context, Key};
use glow::HasContext;

use crate::imgui_instance::ImGuiInstance;

pub struct Window {
    glfw_instance: glfw::Glfw,
    window: glfw::Window,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    imgui_instance: ImGuiInstance,
    gl_context: glow::Context
}

impl Window {
    pub fn new(context: &mut imgui::Context) -> Self {
        let glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, event_receiver) = glfw_instance.create_window(
            1024, 760, "Rusty Edit", 
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window.");

        window.make_current();
        window.set_all_polling(true);

        Self {
            glfw_instance,
            imgui_instance: ImGuiInstance::new(&mut window, context),
            gl_context: unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _) },
            window,
            event_receiver
        }
    }

    pub fn poll_events(&mut self, context: &mut imgui::Context) {
        self.glfw_instance.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            self.imgui_instance.handle_event(context, &event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                _ => {},
            }
        }
    }

    pub fn render_ui<F: Fn(&imgui::Ui)>(&mut self, context: &mut imgui::Context, render: F) {
        self.imgui_instance.render_ui(&mut self.window, context, render);
    }

    pub fn begin_frame(&self) {
        unsafe {
            self.gl_context.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn end_frame(&mut self) {
        self.window.swap_buffers();
    }

    pub fn get_handle(&self) -> &glfw::Window { &self.window }
    pub fn get_mut_handle(&mut self) -> &mut glfw::Window { &mut self.window }
}