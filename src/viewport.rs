use imgui::{Window as ImGuiWindow, Image as ImGuiImage};

use crate::texture::Texture;

pub struct Viewport {
    texture: Option<Texture>,
    id: String
}

impl Viewport {
    pub fn new(id: &str, size: [u32; 2]) -> Self {
        Self {
            texture: Some(Texture::new(size)),
            id: String::from(id)
        }
    }

    fn get_relative_mouse_pos(&self, ui: &imgui::Ui) -> [f32; 2] {
        let min = ui.item_rect_min();
        let max = ui.item_rect_max();
        let mouse_pos = ui.io().mouse_pos;

        let relative_x = mouse_pos[0] - min[0];
        let relative_y = mouse_pos[1] - min[1];
        [
            relative_x.clamp(0.0, max[0] - min[0]),
            relative_y.clamp(0.0, max[1] - min[1])
        ]
    }

    pub fn render_ui(&self, ui: &imgui::Ui) {
        ImGuiWindow::new(&self.id)
            .build(ui, || {
                if let Some(t) = &self.texture {
                    ImGuiImage::new(t.get_imgui_id(), t.get_size_f32())
                        .build(ui);
                    let relative_mouse_pos = self.get_relative_mouse_pos(ui);
                    if ui.is_item_hovered() {
                        // Right click
                        if ui.io().mouse_down[1] {
                            
                        }
                    }
                }
            }
        );
    }

    pub fn get_tex(&self) -> &Option<Texture> { &self.texture }
    pub fn get_mut_tex(&mut self) -> &mut Option<Texture> { &mut self.texture }
}