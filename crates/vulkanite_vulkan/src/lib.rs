extern crate core;

mod adapter;
mod color;
mod command;
mod debug;
mod device;
mod error;
mod instance;
mod pipeline;
mod queue;
mod surface;
mod sync;
mod utils;
mod types;
mod shader;
mod conv;

pub mod vn {
    pub use crate::instance::{Instance, InstanceCreateInfo, InstanceCreationError};
    pub use crate::adapter::Adapter;
    pub use crate::color::Color;
    pub use crate::command::*;
    pub use crate::device::{Device, DeviceCreateInfo};
    pub use crate::queue::{Queue, QueueCreateInfo, QueueFamily};
    pub use crate::surface::{Surface, SurfaceConfig, SurfaceError, Swapchain};
    pub use crate::sync::*;
    pub use crate::types::*;
    pub use crate::utils::Version;
    pub use crate::shader::*;
    pub use crate::pipeline::*;
}

pub mod raw {
    pub use ash::*;
}