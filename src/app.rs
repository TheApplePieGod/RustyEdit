use imgui::{Window as ImGuiWindow};

use crate::asset_manager::AssetManager;
use crate::window::Window;
use crate::viewport::Viewport;

pub struct App {
    window: Window,
    imgui_context: imgui::Context,
    running: bool,
    asset_manager: AssetManager,
    viewport: Viewport
}

impl App {
    pub fn new() -> Self {
        let mut context = imgui::Context::create();
        let window = Window::new(&mut context);

        // Prepare assets
        let mut asset_manager = AssetManager::new();
        asset_manager.load_texture("assets/checkerboard.jpg", "Checkerboard");

        Self {
            running: true,
            asset_manager,
            viewport: Viewport::new("Main Viewport", 500, 500),
            imgui_context: context,
            window 
        }
    }

    pub fn run(&mut self) {
        while self.running && !self.window.get_handle().should_close() {
            self.window.poll_events(&mut self.imgui_context);
            let ui = self.window.begin_frame(&mut self.imgui_context);

            // ui.show_demo_window(&mut true);
            ImGuiWindow::new("Toolbar")
                .no_decoration()
                .position([0.0, 0.0], imgui::Condition::Always)
                .size([1000.0, 50.0], imgui::Condition::Always)
                .build(&ui, || {
                    ui.text("Hallo");
                }
            );
            self.viewport.render_ui(&ui, &self.asset_manager);

            self.window.end_frame(ui);
        }
    }
}