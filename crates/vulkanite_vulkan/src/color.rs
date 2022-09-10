use ash::vk;

// normalized rbga color
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const FULL_TRANSPARENT: Self = Self::norm(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = Self::norm(0.0, 0.0, 0.0, 1.0);
    pub const RED: Self = Self::norm(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::norm(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::norm(0.0, 0.0, 1.0, 1.0);

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 256.0,
            g: g as f32 / 256.0,
            b: b as f32 / 256.0,
            a: 1.0,
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 256.0,
            g: g as f32 / 256.0,
            b: b as f32 / 256.0,
            a,
        }
    }

    pub const fn norm(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self::norm(r, g, b, a)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::norm(r, g, b, 1.0)
    }
}

impl From<(u8, u8, u8, f32)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, f32)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<Color> for vk::ClearValue {
    fn from(val: Color) -> Self {
        vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [val.r, val.g, val.b, val.a],
            },
        }
    }
}
