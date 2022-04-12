use std::{num::{NonZeroU32}, ptr::null, ffi::c_void};
use imgui::{Window as ImGuiWindow, Image as ImGuiImage};

pub struct Viewport {
    image: Option<NonZeroU32>,
    id: String,
    size: [u32; 2]
}

impl Viewport {
    pub fn new(id: &str, size: [u32; 2]) -> Self {
        let mut tex_id: [u32; 1] = [ 0 ];
        let initial_data: Vec<u8> = vec![255; (size[0] * size[1] * 4) as usize];
        unsafe {
            gl::GenTextures(1, tex_id.as_mut_ptr());
            gl::BindTexture(gl::TEXTURE_2D, tex_id[0]);
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, size[0] as i32, size[1] as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, initial_data.as_ptr() as *const c_void);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, size[0] as i32, size[1] as i32);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, size[0] as i32, size[1] as i32, gl::RGBA, gl::UNSIGNED_BYTE, initial_data.as_ptr() as *const c_void);
        }
        Self {
            image: NonZeroU32::new(tex_id[0]),
            id: String::from(id),
            size
        }
    }

    pub fn render_ui(&self, ui: &imgui::Ui) {
        ImGuiWindow::new(&self.id)
            .build(ui, || {
                match self.image {
                    Some(i) => {
                        let tex_id = imgui::TextureId::from(i.get() as usize);
                        ImGuiImage::new(tex_id, [self.size[0] as f32, self.size[1] as f32])
                            .build(ui);
                    },
                    _ => {}
                }
            }
        );
    }
}

impl Drop for Viewport {
    fn drop(&mut self) {
        match self.image {
            Some(i) => unsafe {
                let del: [u32; 1] = [ i.get() ];
                gl::DeleteTextures(1, del.as_ptr());
            }
            _ => {}
        }
    }
}