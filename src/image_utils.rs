use crate::texture::Texture;

pub struct ImageUtils;

impl ImageUtils {
    pub fn draw_square(tex: &mut Texture, x: u32, y: u32, size: u32, color: &[f32]) {
        let width = size * 2 + 1;
        let mut pixels: Vec<u8> = Vec::with_capacity(width.pow(2) as usize);
        for _ in 0..width.pow(2) {
            for c in color {
                pixels.push((*c * 255.0) as u8);
            }
        }

        let mut x_offset = x;
        let mut y_offset = y;
        if size <= x { x_offset -= size; }
        if size <= y { y_offset -= size; }

        tex.update_pixels(x_offset, y_offset, width, width, &pixels);
    }
}