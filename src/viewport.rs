use imgui::{Window as ImGuiWindow, Image as ImGuiImage};
use log::{error, info};

use crate::{texture::Texture, asset_manager::AssetManager, widget_state::WidgetState, image_utils::ImageUtils};

pub struct Viewport {
    texture: Option<Texture>,
    id: String,
    last_mouse_pos: [f32; 2],
    image_just_hovered: bool
}

impl Viewport {
    pub fn new(id: &str, width: u32, height: u32) -> Self {
        let tex_result = Texture::new(width, height, 4, None);

        if let Err(e) = tex_result {
            error!("Failed to create viewport texture: {}", e);            
        }

        Self {
            texture: tex_result.ok(),
            id: String::from(id),
            last_mouse_pos: [0.0, 0.0],
            image_just_hovered: false
        }
    }

    fn get_relative_mouse_pos(ui: &imgui::Ui) -> [f32; 2] {
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

    fn distance(a: &[f32; 2], b: &[f32; 2]) -> f32 {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
    }

    pub fn render_ui(&mut self, ui: &imgui::Ui) {
        ImGuiWindow::new(&self.id)
            .build(ui, || {
                if let Some(t) = &mut self.texture {

                    // Image background
                    {
                        let min = ui.cursor_screen_pos();
                        let max = [min[0] + t.get_width_f32(), min[1] + t.get_height_f32()];
                        match AssetManager::current().borrow().get_texture("Checkerboard") {
                            Some(c) => {
                                ui.get_window_draw_list().add_image(c.get_imgui_id(), min, max)
                                    .uv_max([2.0, 2.0])
                                    .build();
                            },
                            None => {
                                ui.get_window_draw_list().add_rect_filled_multicolor(
                                    min,
                                    max,
                                    [0.0, 0.0, 0.0, 255.0],
                                    [0.0, 0.0, 0.0, 255.0],
                                    [0.0, 0.0, 0.0, 255.0],
                                    [0.0, 0.0, 0.0, 255.0]
                                )
                            }
                        }
                    }

                    // Working image
                    ImGuiImage::new(t.get_imgui_id(), t.get_size_f32())
                        .build(ui);
                    let relative_mouse_pos = Viewport::get_relative_mouse_pos(ui);
                    let image_hovered = ui.is_item_hovered();
                    if image_hovered || self.image_just_hovered {
                        // Right click
                        if ui.io().mouse_down[1] {
                            let step_count = Viewport::distance(&self.last_mouse_pos, &relative_mouse_pos) as u32;
                            let slope = [
                                (relative_mouse_pos[0] - self.last_mouse_pos[0]) / step_count as f32,
                                (relative_mouse_pos[1] - self.last_mouse_pos[1]) / step_count as f32
                            ];
                            let mut current_pos = self.last_mouse_pos;
                            for _ in 0..step_count {
                                ImageUtils::draw_square(t, current_pos[0] as u32, current_pos[1] as u32, 3, &WidgetState::current().borrow().primary_color);
                                current_pos[0] += slope[0];
                                current_pos[1] += slope[1];
                            }
                        }
                    }

                    if ui.button("Clear") {
                        t.clear(&[0, 0, 0, 0]);
                    }

                    self.image_just_hovered = image_hovered;
                    self.last_mouse_pos = relative_mouse_pos;
                }
            }
        );
    }

    pub fn draw_square(&self, tex: &mut Texture, size: u32) {
        
    }

    pub fn export(&self, path: &str) {
        if let Some(t) = &self.texture {
            match t.export(path) {
                Ok(_) => info!("Exported current texture"),
                Err(e) => error!("Failed to export current texture: {}", e)
            }
        }
    }

    pub fn get_tex(&self) -> &Option<Texture> { &self.texture }
    pub fn get_mut_tex(&mut self) -> &mut Option<Texture> { &mut self.texture }
}