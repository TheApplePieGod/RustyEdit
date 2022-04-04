use crate::window::Window;

pub struct AppBuilder {
    window: Option<Window>,
    imgui_context: imgui::Context,
}

impl AppBuilder {
    pub fn new() -> Self {
        let mut context = imgui::Context::create();
        context.set_ini_filename(None);

        Self {
            window: None,
            imgui_context: context
        }
    }

    pub fn window(mut self) -> Self {
        self.window = Some(Window::new(&mut self.imgui_context));
        self
    }

    pub fn build(self) -> App {
        let window = self.window.expect("Must build the app with a call to window()");
        App {
            window,
            imgui_context: self.imgui_context,
            running: true
        }
    }
}

pub struct App {
    window: Window,
    imgui_context: imgui::Context,
    running: bool
}

impl App {
    pub fn run(&mut self) {
        while self.running && !self.window.get_handle().should_close() {
            self.window.poll_events(&mut self.imgui_context);

            self.window.render_ui(&mut self.imgui_context, |ui| {
                ui.show_demo_window(&mut true);
            });

            self.window.end_frame();
        }
    }
}