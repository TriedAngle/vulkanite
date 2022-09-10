mod device;
mod image;
mod pipeline;

pub use device::*;
pub use image::*;
pub use pipeline::*;

use std::ffi::CString;

#[derive(Debug, Copy, Clone)]
pub struct Layers {
    khronos_validation: bool,
}

impl Layers {
    pub fn none() -> Self {
        Self {
            khronos_validation: false,
        }
    }
}

impl From<Layers> for Vec<CString> {
    fn from(layers: Layers) -> Self {
        let mut result = vec![];

        if layers.khronos_validation {
            result.push(CString::new(&"VK_LAYER_KHRONOS_validation"[..]).unwrap())
        }

        result
    }
}

impl Default for Layers {
    fn default() -> Self {
        Self {
            khronos_validation: true,
        }
    }
}

#[cfg(test)]
mod type_tests {
    use super::*;
    #[test]
    fn test() {
        let features = Features {
            shader_cull_distance: true,
            ..Features::none()
        };

        let extensions = Extensions {
            vk_khr_surface: true,
            ..Extensions::none()
        };
    }
}
