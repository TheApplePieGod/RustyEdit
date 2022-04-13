use imgui::{Window as ImGuiWindow, MenuItem, StyleVar, ColorEdit, Drag};

use crate::app_state::AppState;
use crate::viewport::Viewport;
use crate::widget_state::WidgetState;

pub struct WidgetManager {
    viewport: Viewport,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            viewport: Viewport::new("Main Viewport", 500, 500),
        }
    }

    pub fn render_ui(&mut self, ui: &imgui::Ui) {
        let padding_var = ui.push_style_var(StyleVar::WindowPadding([5.0, 5.0]));

        // Main menu bar
        ui.main_menu_bar(|| {
            ui.menu("File", || {
                if MenuItem::new("Export").build(&ui) {
                    self.viewport.export("export/test.png");
                }
                if MenuItem::new("Quit").build(&ui) {
                    AppState::current().borrow_mut().running = false;
                }
            });
        });          

        // Toolbar
        ImGuiWindow::new("Toolbar")
            //.no_decoration()
            //.position([0.0, 0.0], imgui::Condition::Always)
            //.size([1000.0, 50.0], imgui::Condition::Always)
            .build(&ui, || {
                ui.text("Thickness");
                ui.same_line();
                Drag::new("##Thickness")
                    .range(0u32, 50)
                    .speed(0.2)
                    .build(ui, &mut WidgetState::current().borrow_mut().thickness);
            }
        );

        // Main viewport
        self.viewport.render_ui(&ui);

        // Color picker
        ImGuiWindow::new("Color Picker")
            .build(&ui, || {
                let mut colors: [f32; 4] = WidgetState::current().borrow().primary_color.into();
                ColorEdit::new("##GlobalColor", &mut colors)
                    .alpha(true)
                    .alpha_bar(true)
                    .build(&ui);
                WidgetState::current().borrow_mut().primary_color = colors.into();
            }
        );

        padding_var.pop();
    }
}