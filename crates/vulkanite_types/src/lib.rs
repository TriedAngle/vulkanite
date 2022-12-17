mod buffer;
mod image;
mod pipeline;
mod texture;

pub use buffer::*;
pub use image::*;
pub use pipeline::*;
pub use texture::*;

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct TextureUsages: u32 {
        const TRANSFER_SRC = 1 << 0;
        const TRANSFER_DST = 1 << 1;
        const SAMPLED = 1 << 2;
        const STORAGE = 1 << 3;
        const COLOR_ATTACHMENT = 1 << 4;
        const DEPTH_STENCIL_ATTACHMENT = 1 << 5;
        const TRANSIENT_ATTACHMENT = 1 << 6;
        const INPUT_ATTACHMENT = 1 << 7;
    }

    #[repr(transparent)]
    pub struct TextureAspects: u32 {
        const COLOR = 1 << 0;
        const DEPTH = 1 << 1;
        const STENCIL = 1 << 2;
        const METADATA = 1 << 3;
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PresentMode {
    Immediate = 0,
    Mailbox = 1,
    Fifo = 2,
    FifoRelaxed = 3,
}

impl From<i32> for PresentMode {
    fn from(val: i32) -> Self {
        match val {
            0 => Self::Immediate,
            1 => Self::Mailbox,
            2 => Self::Fifo,
            3 => Self::FifoRelaxed,
            _ => panic!("Invalid value for Present Mode"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum TextureDimension {
    D1,
    D2,
    D3,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum TextureViewDimension {
    D1,
    D2,
    D3,
    Cube,
    Array1D,
    Array2D,
    ArrayCube,
}

#[derive(Debug, Copy, Clone)]
pub enum SharingMode {
    Exclusive,
    Concurrent,
}

#[derive(Debug, Copy, Clone)]
pub struct ImageSubresourceRange {
    pub aspects: TextureAspects,
    pub base_mip_level: u32,
    pub mip_level_count: u32,
    pub base_array_layer: u32,
    pub array_layer_count: u32,
}
