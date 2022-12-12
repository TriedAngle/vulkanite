use crate::device::{Device, DeviceError, DeviceShared};
use ash::vk;
use naga::back::spv;
use naga::back::spv::WriterFlags;
use naga::front::wgsl;
use naga::valid::{Capabilities, ValidationFlags, Validator};
use shaderc::{CompileOptions, SpirvVersion};
use std::borrow::Cow;
use std::io;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Default, Copy, Clone)]
pub struct WgslShaderCompileInfo {
    debug: bool,
    flip_y: bool,
}

pub enum ShaderSource<'a> {
    Wgsl(Cow<'a, str>, WgslShaderCompileInfo),
    Glsl {
        content: Cow<'a, str>,
        kind: ShaderKind,
        entry: &'a str,
    },
    Hlsl {
        content: Cow<'a, str>,
        kind: ShaderKind,
        entry: &'a str,
    },
    SpirV(&'a [u8]),
}

pub struct ShaderModule {
    pub(crate) handle: vk::ShaderModule,
}

pub enum ShaderKind {
    Vertex,
    Fragment,
    Compute,
    Geometry,
    // requires #pragma in source code
    InferFromSource,
    RayGeneration,
    AnyHit,
    ClosestHit,
    Miss,
    Intersection,
    Callable,
    Task,
    Mesh,
}

fn shader_stage_to_shaderc(kind: ShaderKind) -> shaderc::ShaderKind {
    match kind {
        ShaderKind::Vertex => shaderc::ShaderKind::Vertex,
        ShaderKind::Fragment => shaderc::ShaderKind::Fragment,
        ShaderKind::Compute => shaderc::ShaderKind::Compute,
        ShaderKind::Geometry => shaderc::ShaderKind::Geometry,
        ShaderKind::InferFromSource => shaderc::ShaderKind::InferFromSource,
        ShaderKind::RayGeneration => shaderc::ShaderKind::RayGeneration,
        ShaderKind::AnyHit => shaderc::ShaderKind::AnyHit,
        ShaderKind::ClosestHit => shaderc::ShaderKind::ClosestHit,
        ShaderKind::Miss => shaderc::ShaderKind::Miss,
        ShaderKind::Intersection => shaderc::ShaderKind::Intersection,
        ShaderKind::Callable => shaderc::ShaderKind::Callable,
        ShaderKind::Task => shaderc::ShaderKind::Task,
        ShaderKind::Mesh => shaderc::ShaderKind::Mesh,
    }
}

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error(transparent)]
    WgslParse(#[from] wgsl::ParseError),
    #[error("GLSL Parse Error: {0:?}")]
    GlslParse(#[from] shaderc::Error),
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
    ) -> Result<ShaderModule, ShaderError> {
        match source {
            ShaderSource::Wgsl(source, info) => {
                let module = wgsl::parse_str(&source).map_err(|e| ShaderError::WgslParse(e))?;
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

                let spv = spv::write_vec(&module, &info, &opts, None)
                    .map_err(|e| ShaderError::SpirVParse(e))?;

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

                Ok(ShaderModule { handle })
            }

            ShaderSource::Glsl {
                content,
                kind: stage,
                entry,
            } => {
                let compiler = shaderc::Compiler::new().unwrap();

                let artifact = compiler
                    .compile_into_spirv(&content, shader_stage_to_shaderc(stage), "", entry, None)
                    .map_err(|e| ShaderError::GlslParse(e))?;

                let vk_info = vk::ShaderModuleCreateInfo::builder()
                    .flags(vk::ShaderModuleCreateFlags::empty())
                    .code(&artifact.as_binary());

                let handle = unsafe {
                    self.shared
                        .handle
                        .create_shader_module(&vk_info, None)
                        .map_err(DeviceError::Other)
                        .map_err(ShaderError::Device)?
                };

                Ok(ShaderModule { handle })
            }
            ShaderSource::Hlsl { .. } => {
                unimplemented!()
            }
            ShaderSource::SpirV(spirv_bytes) => {
                let mut cursor = io::Cursor::new(spirv_bytes);
                let spirv = ash::util::read_spv(&mut cursor).unwrap();

                let vk_info = vk::ShaderModuleCreateInfo::builder()
                    .flags(vk::ShaderModuleCreateFlags::empty())
                    .code(&spirv);

                let handle = unsafe {
                    self.shared
                        .handle
                        .create_shader_module(&vk_info, None)
                        .map_err(DeviceError::Other)
                        .map_err(ShaderError::Device)?
                };

                Ok(ShaderModule { handle })
            }
        }
    }
}
