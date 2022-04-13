use crate::app_state::AppState;
use crate::asset_manager::AssetManager;
use crate::widget_manager::WidgetManager;
use crate::window::Window;

pub struct App {
    window: Window,
    imgui_context: imgui::Context,
    widget_manager: WidgetManager
}

impl App {
    pub fn new() -> Self {
        let mut context = imgui::Context::create();
        let window = Window::new(&mut context);

        // Prepare assets
        AssetManager::current().borrow_mut().load_texture("assets/checkerboard.jpg", "Checkerboard");

        Self {
            widget_manager: WidgetManager::new(),
            imgui_context: context,
            window 
        }
    }

    fn render_ui(&mut self, ui: &imgui::Ui) {
        
    }

    pub fn run(&mut self) {
        while AppState::current().borrow().running && !self.window.get_handle().should_close() {
            self.window.poll_events(&mut self.imgui_context);
            let ui = self.window.begin_frame(&mut self.imgui_context);

            self.widget_manager.render_ui(&ui);
            
            self.window.end_frame(ui);
        }
    }
}