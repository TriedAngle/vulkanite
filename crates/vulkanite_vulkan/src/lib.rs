extern crate core;

mod adapter;
mod buffer;
mod color;
mod command;
mod conv;
mod debug;
mod device;
mod error;
mod instance;
mod pipeline;
mod queue;
mod shader;
mod surface;
mod sync;
mod types;
mod utils;

pub mod vn {
    pub use crate::adapter::Adapter;
    pub use crate::buffer::{Buffer, BufferInitInfo};
    pub use crate::color::Color;
    pub use crate::command::*;
    pub use crate::device::{Device, DeviceCreateInfo, DeviceError};
    pub use crate::instance::{Instance, InstanceCreateInfo, InstanceCreationError};
    pub use crate::pipeline::*;
    pub use crate::queue::{Queue, QueueCreateInfo, QueueFamily};
    pub use crate::shader::*;
    pub use crate::surface::{Surface, SurfaceConfig, SurfaceError, Swapchain};
    pub use crate::sync::*;
    pub use crate::types::*;
    pub use crate::utils::Version;
    pub use vulkanite_types::*;
}

pub mod raw {
    pub use ash::*;
}
