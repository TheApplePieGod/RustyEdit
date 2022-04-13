#[derive(Copy, Clone)]
pub enum Color {
    Float4 { r: f32, g: f32, b: f32, a: f32 },
    Byte4 { r: u8, g: u8, b: u8, a: u8 }
}

impl Color {
    pub fn r_f32(&self) -> f32 {
        match self {
            Self::Float4 { r, .. } => *r,
            Self::Byte4 { r, .. } => *r as f32 / 255.0
        }
    }

    pub fn g_f32(&self) -> f32 {
        match self {
            Self::Float4 { g, .. } => *g,
            Self::Byte4 { g, .. } => *g as f32 / 255.0
        }
    }

    pub fn b_f32(&self) -> f32 {
        match self {
            Self::Float4 { b, .. } => *b,
            Self::Byte4 { b, .. } => *b as f32 / 255.0
        }
    }

    pub fn a_f32(&self) -> f32 {
        match self {
            Self::Float4 { a, .. } => *a,
            Self::Byte4 { a, .. } => *a as f32 / 255.0
        }
    }

    pub fn r_u8(&self) -> u8 {
        match self {
            Self::Float4 { r, .. } => (*r * 255.0) as u8,
            Self::Byte4 { r, .. } => *r
        }
    }

    pub fn g_u8(&self) -> u8 {
        match self {
            Self::Float4 { g, .. } => (*g * 255.0) as u8,
            Self::Byte4 { g, .. } => *g

        }
    }

    pub fn b_u8(&self) -> u8 {
        match self {
            Self::Float4 { b, .. } => (*b * 255.0) as u8,
            Self::Byte4 { b, .. } => *b

        }
    }

    pub fn a_u8(&self) -> u8 {
        match self {
            Self::Float4 { a, .. } => (*a * 255.0) as u8,
            Self::Byte4 { a, .. } => *a

        }
    }
}

impl From<&[f32]> for Color {
    fn from(item: &[f32]) -> Self {
        assert!(item.len() == 4);
        Color::Float4 {
            r: item[0],
            g: item[1],
            b: item[2],
            a: item[3]
        }
    }
}

impl From<&[u8]> for Color {
    fn from(item: &[u8]) -> Self {
        assert!(item.len() == 4);
        Color::Byte4 {
            r: item[0],
            g: item[1],
            b: item[2],
            a: item[3]
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(item: [f32; 4]) -> Self {
        Color::Float4 {
            r: item[0],
            g: item[1],
            b: item[2],
            a: item[3]
        }
    }
}

impl From<[u8; 4]> for Color {
    fn from(item: [u8; 4]) -> Self {
        Color::Byte4 {
            r: item[0],
            g: item[1],
            b: item[2],
            a: item[3]
        }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            self.r_f32(),
            self.g_f32(),
            self.b_f32(),
            self.a_f32()
        ]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [
            self.r_u8(),
            self.g_u8(),
            self.b_u8(),
            self.a_u8()
        ]
    }
}