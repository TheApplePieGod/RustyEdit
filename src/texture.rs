use std::{ffi::c_void};

pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
    pixels: Vec<u8>
}

impl Texture {
    pub fn new(size: [u32; 2]) -> Self {
        let mut tex_id: [u32; 1] = [ 0 ];
        let initial_data: Vec<u8> = vec![255; (size[0] * size[1] * 4) as usize];
        unsafe {
            gl::GenTextures(1, tex_id.as_mut_ptr());
            gl::BindTexture(gl::TEXTURE_2D, tex_id[0]);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, size[0] as i32, size[1] as i32);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, size[0] as i32, size[1] as i32, gl::RGBA, gl::UNSIGNED_BYTE, initial_data.as_ptr() as *const c_void);
        }
        Self {
            id: tex_id[0],
            width: size[0],
            height: size[1],
            pixels: initial_data
        }
    }

    pub fn update_pixel(&mut self, x_offset: u32, y_offset: u32, data: u8) {
        let update: [u8; 1] = [ data ];
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D, 
                0, 
                x_offset as i32, 
                y_offset as i32, 
                1, 
                1, 
                gl::RGBA, 
                gl::UNSIGNED_BYTE, 
                update.as_ptr() as *const c_void);
        }
    }

    pub fn update_pixels(&mut self, x_offset: u32, y_offset: u32, width: u32, height: u32, data: &[u8]) {
        let mut data_index = 0;
        for i in 0..height {
            for j in 0..width * 4 {
                let arr_index = ((y_offset + i) * self.width * 4 + (x_offset * 4 + j)) as usize;
                self.pixels[arr_index] = data[data_index];
                data_index += 1;
            }
        }

        // Debug print
        // for i in 0..self.height {
        //     for j in 0..self.width * 4 {
        //         let index = (i * self.width * 4 + j) as usize;
        //         match self.pixels[index] {
        //             255 => print!("X"),
        //             50 => print!("O"),
        //             _ => {}
        //         }
        //     }
        //     print!("\n");
        // }

        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D, 
                0, 
                x_offset as i32, 
                y_offset as i32, 
                width as i32, 
                height as i32, 
                gl::RGBA, 
                gl::UNSIGNED_BYTE, 
                data.as_ptr() as *const c_void);
        }
    }

    pub fn get_id(&self) -> u32 { self.id }
    pub fn get_imgui_id(&self) -> imgui::TextureId { imgui::TextureId::from(self.id as usize) }
    pub fn get_size(&self) -> [u32; 2] { [self.width, self.height] }
    pub fn get_size_f32(&self) -> [f32; 2] { [self.width as f32, self.height as f32] }
    pub fn get_width(&self) -> u32 { self.width }
    pub fn get_height(&self) -> u32 { self.height }
}

impl Drop for Texture {
    fn drop(&mut self) {
        let del: [u32; 1] = [ self.id ];
        unsafe {
            gl::DeleteTextures(1, del.as_ptr());
        }
    }
}