bitflags::bitflags! {
    pub struct BufferUsages: u32 {
        const MAP_READ = 1 << 0;
        const MAP_WRITE = 1 << 1;
        const COPY_SRC = 1 << 2;
        const COPY_DST = 1 << 3;
        const INDEX = 1 << 4;
        const VERTEX = 1 << 5;
        const UNIFORM = 1 << 6;
        const STORAGE_READ = 1 << 7;
        const STORAGE_READ_WRITE = 1 << 8;
        const INDIRECT = 1 << 9;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BufferSharing {
    Exclusive,
    Concurrent,
}

pub type BufferAddress = u64;
pub const COPY_BUFFER_ALIGNMENT: BufferAddress = 4;

// #[derive(Debug, Copy, Clone)]
// pub enum BufferMemoryLocation {
//     Unknown,
//     CpuToGpu,
//     GpuToCpu,
//     GpuOnly,
// }
