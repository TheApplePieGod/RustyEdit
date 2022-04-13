use crate::{texture::Texture, color::Color};

pub struct ImageUtils;

impl ImageUtils {
    pub fn additive_blend(src: &Color, dst: &Color) -> [u8; 4] {
        let mut res: Color = Color::Float4 { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
        if let Color::Float4 { r, g, b, a } = &mut res {
            *a = 1.0 - (1.0 - src.a_f32()) * (1.0 - dst.a_f32());
            if *a < 1.0e-6 { return res.into() } // fully transparent
            *r = src.r_f32() * src.a_f32() / *a + dst.r_f32() * dst.a_f32() * (1.0 - src.a_f32()) / *a;
            *g = src.g_f32() * src.a_f32() / *a + dst.g_f32() * dst.a_f32() * (1.0 - src.a_f32()) / *a;
            *b = src.b_f32() * src.a_f32() / *a + dst.b_f32() * dst.a_f32() * (1.0 - src.a_f32()) / *a;
        }

        res.into()
    }

    pub fn draw_square(tex: &mut Texture, x: u32, y: u32, size: u32, color: &Color) {
        let width = size * 2 + 1;
        let mut pixels: Vec<u8> = Vec::with_capacity(width.pow(2) as usize);
        let mut x_offset = x;
        let mut y_offset = y;

        // todo: also decrease size here or something
        if size <= x { x_offset -= size; }
        if size <= y { y_offset -= size; }

        for i in 0..width as usize {
            for j in 0..width as usize {
                let pix_idx = tex.get_pixel_index(j as u32 + x_offset, i as u32 + y_offset);
                if pix_idx >= tex.get_pixels().len() { continue; }
                let final_color = ImageUtils::additive_blend(
                    color, 
                    &tex.get_pixels()[pix_idx..pix_idx+4].into()
                );
                for c in final_color {
                    pixels.push(c);
                }
            }
        }

        tex.update_pixels(x_offset, y_offset, width, width, &pixels);
    }
}