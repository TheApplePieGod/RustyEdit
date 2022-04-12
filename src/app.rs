use imgui::{Window as ImGuiWindow};

use crate::window::Window;
use crate::viewport::Viewport;

pub struct App {
    window: Window,
    imgui_context: imgui::Context,
    running: bool,
    viewport: Viewport
}

impl App {
    pub fn new() -> Self {
        let mut context = imgui::Context::create();
        let window = Window::new(&mut context);

        Self {
            running: true,
            viewport: Viewport::new("Main Viewport", [500, 500]),
            imgui_context: context,
            window 
        }
    }

    pub fn run(&mut self) {
        while self.running && !self.window.get_handle().should_close() {
            self.window.poll_events(&mut self.imgui_context);
            self.window.begin_frame();

            self.window.render_ui(&mut self.imgui_context, |ui| {
                // ui.show_demo_window(&mut true);
                ImGuiWindow::new("Toolbar")
                    .no_decoration()
                    .position([0.0, 0.0], imgui::Condition::Always)
                    .size([1000.0, 50.0], imgui::Condition::Always)
                    .build(ui, || {
                        ui.text("Hallo");
                    }
                );
                self.viewport.render_ui(ui);
            });

            self.window.end_frame();
        }
    }
}