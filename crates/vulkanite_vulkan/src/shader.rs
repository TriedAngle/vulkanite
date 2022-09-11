use crate::device::{Device, DeviceError, DeviceShared};
use ash::vk;
use naga::back::spv;
use naga::back::spv::WriterFlags;
use naga::front::glsl;
use naga::front::wgsl;
use naga::valid::{Capabilities, ValidationFlags, Validator};
use naga::{back, Module};
use std::borrow::Cow;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Default, Copy, Clone)]
pub struct ShaderCompileInfo {
    debug: bool,
    flip_y: bool,
}

pub enum ShaderSource<'a> {
    Wgsl(Cow<'a, str>),
    SpirV(Cow<'a, str>),
}

pub struct ShaderModule {
    pub(crate) handle: vk::ShaderModule,
    pub(crate) device: Arc<DeviceShared>,
}

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error(transparent)]
    WgslParse(#[from] wgsl::ParseError),
    #[error("GLSL Parse Error: {0:?}")]
    GlslParse(Vec<glsl::Error>),
    #[error(transparent)]
    SpirVParse(#[from] spv::Error),
    #[error(transparent)]
    Validation(#[from] naga::WithSpan<naga::valid::ValidationError>),
    #[error(transparent)]
    Device(#[from] DeviceError),
}

impl Device {
    pub fn create_shader_module(
        &self,
        source: ShaderSource<'_>,
        info: ShaderCompileInfo,
    ) -> Result<ShaderModule, ShaderError> {
        let module = match source {
            ShaderSource::Wgsl(source) => {
                wgsl::parse_str(&source).map_err(|e| ShaderError::WgslParse(e))
            }
            ShaderSource::SpirV(spirv) => {
                unimplemented!();
            }
        }?;

        let mut opts = spv::Options::default();

        if info.debug {
            opts.flags.insert(WriterFlags::DEBUG);
        }
        if info.flip_y {
            opts.flags.insert(WriterFlags::ADJUST_COORDINATE_SPACE);
        }

        let info = Validator::new(ValidationFlags::all(), Capabilities::all())
            .validate(&module)
            .map_err(|e| ShaderError::Validation(e))?;

        let spv =
            spv::write_vec(&module, &info, &opts, None).map_err(|e| ShaderError::SpirVParse(e))?;
        let vk_info = vk::ShaderModuleCreateInfo::builder()
            .flags(vk::ShaderModuleCreateFlags::empty())
            .code(&spv);

        let handle = unsafe {
            self.shared
                .handle
                .create_shader_module(&vk_info, None)
                .map_err(DeviceError::Other)
                .map_err(ShaderError::Device)?
        };

        Ok(ShaderModule {
            handle,
            device: self.shared.clone(),
        })
    }
}
