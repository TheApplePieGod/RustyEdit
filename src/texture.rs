use std::{ffi::c_void};

pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
    channels: u32,
    pixels: Vec<u8>,
    format: gl::types::GLenum,
    internal_format: gl::types::GLenum
}

impl Texture {
    pub fn new(width: u32, height: u32, channels: u32, initial_data: Option<&[u8]>) -> Result<Self, &str> {
        let mut tex_id: [u32; 1] = [ 0 ];
        let data: Vec<u8>;
        match initial_data {
            Some(d) => {
                data = d.to_vec();    
            },
            None => {
                data = vec![0; (width * height * channels) as usize];
            }
        }

        let format = Texture::get_format_from_channels(channels);
        let internal_format = Texture::get_internal_format_from_channels(channels);

        if format.is_none() || internal_format.is_none() {
            return Err("Invalid channel count");
        }

        unsafe {
            gl::GenTextures(1, tex_id.as_mut_ptr());
            gl::BindTexture(gl::TEXTURE_2D, tex_id[0]);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, internal_format.unwrap(), width as i32, height as i32);
            gl::TexSubImage2D(
                gl::TEXTURE_2D, 
                0, 
                0, 
                0, 
                width as i32, 
                height as i32, 
                format.unwrap(), 
                gl::UNSIGNED_BYTE, 
                data.as_ptr() as *const c_void
            );
        }

        Ok(Self {
            id: tex_id[0],
            width,
            height,
            channels,
            pixels: data,
            format: format.unwrap(),
            internal_format: internal_format.unwrap()
        })
    }

    fn get_format_from_channels(channels: u32) -> Option<gl::types::GLenum> {
        match channels {
            1 => Some(gl::RED),
            2 => Some(gl::RG),
            3 => Some(gl::RGB),
            4 => Some(gl::RGBA),
            _ => None
        }
    }

    fn get_internal_format_from_channels(channels: u32) -> Option<gl::types::GLenum> {
        match channels {
            1 => Some(gl::R8),
            2 => Some(gl::RG8),
            3 => Some(gl::RGB8),
            4 => Some(gl::RGBA8),
            _ => None
        }
    }

    fn get_color_type_from_channels(channels: u32) -> Option<image::ColorType> {
        match channels {
            3 => Some(image::ColorType::Rgb8),
            4 => Some(image::ColorType::Rgba8),
            _ => None
        }
    }

    pub fn clear(&mut self, color: &[u8]) {
        let size = (self.width * self.height * self.channels) as usize;
        for i in 0..size {
            self.pixels[i] = color[i % self.channels as usize];
        }

        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D, 
                0, 
                0,
                0, 
                self.width as i32, 
                self.height as i32,
                self.format,
                gl::UNSIGNED_BYTE, 
                self.pixels.as_ptr() as *const c_void
            );
        }
    }

    pub fn update_pixel(&mut self, x_offset: u32, y_offset: u32, color: &[u8; 4]) {
        self.update_pixels(x_offset, y_offset, 1, 1, color);
    }

    pub fn update_pixels(&mut self, x_offset: u32, y_offset: u32, width: u32, height: u32, data: &[u8]) {
        if x_offset + width > self.width { return; }
        if y_offset + height > self.height { return; }

        let mut data_index = 0;
        for i in 0..height {
            for j in 0..width * self.channels {
                let arr_index = ((y_offset + i) * self.width * self.channels + (x_offset * self.channels + j)) as usize;
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
                self.format, 
                gl::UNSIGNED_BYTE, 
                data.as_ptr() as *const c_void
            );
        }
    }

    pub fn export(&self, path: &str) -> Result<(), String> {
        match Texture::get_color_type_from_channels(self.channels) {
            Some(t) => {
                let res = image::save_buffer(path, &self.pixels, self.width, self.height, t);
                if let Err(e) = res {
                    Err(e.to_string())
                } else {
                    Ok(())
                }
            },
            None => Err(String::from("Unsupported channel count"))
        }
    }

    pub fn get_pixel_index(&self, x: u32, y: u32) -> usize {
        ((x * self.channels) + (y * self.channels * self.width)) as usize
    }

    pub fn get_id(&self) -> u32 { self.id }
    pub fn get_imgui_id(&self) -> imgui::TextureId { imgui::TextureId::from(self.id as usize) }
    pub fn get_size(&self) -> [u32; 2] { [self.width, self.height] }
    pub fn get_size_f32(&self) -> [f32; 2] { [self.width as f32, self.height as f32] }
    pub fn get_width(&self) -> u32 { self.width }
    pub fn get_height(&self) -> u32 { self.height }
    pub fn get_width_f32(&self) -> f32 { self.width as f32 }
    pub fn get_height_f32(&self) -> f32 { self.height as f32 }
    pub fn get_channels(&self) -> u32 { self.channels }
    pub fn get_pixels(&self) -> &Vec<u8> { &self.pixels }
}

impl Drop for Texture {
    fn drop(&mut self) {
        let del: [u32; 1] = [ self.id ];
        unsafe {
            gl::DeleteTextures(1, del.as_ptr());
        }
    }
}