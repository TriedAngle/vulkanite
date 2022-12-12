mod buffer;
mod pipeline;

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

pub use buffer::*;
pub use pipeline::*;
