use crate::conv;
use crate::device::{Device, DeviceError, DeviceShared};
use crate::shader::ShaderModule;
use crate::types::TextureFormat;
use ash::vk;
use ash::vk::PipelineVertexInputStateCreateFlags;
use std::collections::BTreeMap;
use std::num::NonZeroU32;
use std::ops::Range;
use std::sync::Arc;
use std::{ffi, ptr};
pub use vulkanite_types::pipeline::{BlendState, ColorWrites, VertexFormat};

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ShaderStages: u32 {
        /// Binding is not visible from any shader stage.
        const NONE = 0;
        /// Binding is visible from the vertex shader of a render pipeline.
        const VERTEX = 1 << 0;
        /// Binding is visible from the fragment shader of a render pipeline.
        const FRAGMENT = 1 << 1;
        /// Binding is visible from the compute shader of a compute pipeline.
        const COMPUTE = 1 << 2;
        /// Binding is visible from the vertex and fragment shaders of a render pipeline.
        const VERTEX_FRAGMENT = Self::VERTEX.bits | Self::FRAGMENT.bits;
    }

    #[repr(transparent)]
    pub struct PipelineFlags: u32 {
        const BLEND_CONSTANT = 1 << 0;
        const STENCIL_REFERENCE = 1 << 1;
        const WRITES_DEPTH_STENCIL = 1 << 2;
    }

    /// Pipeline layout creation flags.
    pub struct PipelineLayoutFlags: u32 {
        /// Include support for base vertex/instance drawing.
        const BASE_VERTEX_INSTANCE = 1 << 0;
        /// Include support for num work groups builtin.
        const NUM_WORK_GROUPS = 1 << 1;
    }

    #[repr(transparent)]
    pub struct CullModeFlags: u32 {
        const NONE = 0;
        const FRONT = 1 << 0;
        const BACK = 1 << 1;
        const FRONT_BACK = 1 << 2;
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
    FillRectangleNV,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    LineListWithAdjacency = 6,
    LineStripWithAdjacency = 7,
    TriangleListWithAdjacency = 8,
    TriangleStripWithAdjacency = 9,
    PatchList = 10,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum FrontFace {
    CounterClock = 0,
    Clock = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum IndexFormat {
    Uint16 = 0,
    Uint32 = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DepthCompareOperator {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum VertexStepMode {
    Vertex = 0,
    Instance = 1,
}

pub struct PipelineLayout {
    pub(crate) handle: vk::PipelineLayout,
    pub(crate) binding_arrays: naga::back::spv::BindingMap,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PushConstantRange {
    /// Stage push constant range is visible from. Each stage can only be served by at most one range.
    /// One range can serve multiple stages however.
    pub stages: ShaderStages,
    /// Range in push constant memory to use for the stage. Must be less than [`Limits::max_push_constant_size`].
    /// Start and end must be aligned to the 4s.
    pub range: Range<u32>,
}

#[derive(Debug)]
pub struct BindGroupLayout {
    handle: vk::DescriptorSetLayout,
}

pub struct PipelineLayoutInfo<'a> {
    pub flags: PipelineLayoutFlags,
    pub bind_group_layouts: &'a [&'a BindGroupLayout],
    pub push_constant_ranges: &'a [PushConstantRange],
}

pub struct BindGroupLayoutDescriptor<'a> {
    entries: &'a [BindGroupLayoutEntry],
}

pub struct BindGroupLayoutEntry {
    binding: u32,
    visibility: ShaderStages,
    ty: BindingType,
    count: Option<NonZeroU32>,
}

pub enum BindingType {}

pub struct ShaderStage<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: &'a str,
}

pub struct FragmentState<'a> {
    module: &'a ShaderModule,
}

pub struct PrimitiveState {
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub front_face: FrontFace,
    pub cull_mode: Option<CullModeFlags>,
    pub polygon_mode: PolygonMode,
    pub unclipped_depth: bool,
    pub conservative: bool,
    pub line_width: f32,
}

pub struct DepthStencilState {
    pub format: TextureFormat,
    pub depth_write_enabled: bool,
    pub depth_compare: DepthCompareOperator,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MultisampleState {
    pub count: u32,
    pub mask: u64,
    pub alpha_to_coverage_enabled: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorTargetState {
    pub format: TextureFormat,
    pub blend: Option<BlendState>,
    pub write_mask: ColorWrites,
}

pub type BufferAddress = u64;
pub type ShaderLocation = u32;

pub struct VertexAttribute {
    pub format: VertexFormat,
    pub offset: BufferAddress,
    pub shader_location: ShaderLocation,
}

pub struct VertexBufferLayout<'a> {
    pub array_stride: BufferAddress,
    pub step_mode: VertexStepMode,
    pub attributes: &'a [VertexAttribute],
}

pub struct RasterPipelineInfo<'a> {
    pub layout: &'a PipelineLayout,
    pub vertex: ShaderStage<'a>,
    pub vertex_buffers: Option<&'a [VertexBufferLayout<'a>]>,
    pub fragment: Option<ShaderStage<'a>>,
    pub primitive: PrimitiveState,
    // pub depth_stencil:
    pub multisample: MultisampleState,
    pub targets: &'a [ColorTargetState],
}

pub struct ComputePipelineInfo {}

pub struct RasterPipeline {
    pub(crate) device: Arc<DeviceShared>,
    pub(crate) handle: vk::Pipeline,
}

pub struct ComputePipeline {
    pub(crate) device: Arc<DeviceShared>,
    pub(crate) handle: vk::Pipeline,
}

impl Device {
    // pub fn create_bindgroup_layout(&self) -> Result<BindGroupLayout, DeviceError> {
    //
    // }

    pub fn create_pipeline_layout(
        &self,
        info: &PipelineLayoutInfo,
    ) -> Result<PipelineLayout, DeviceError> {
        let vk_set_layouts = info
            .bind_group_layouts
            .iter()
            .map(|bgl| bgl.handle)
            .collect::<Vec<_>>();

        let vk_push_constant_ranges = info
            .push_constant_ranges
            .iter()
            .map(|pcr| vk::PushConstantRange {
                stage_flags: conv::map_shader_stage(pcr.stages),
                offset: pcr.range.start,
                size: pcr.range.end - pcr.range.start,
            })
            .collect::<Vec<_>>();

        let layout_info = vk::PipelineLayoutCreateInfo::builder()
            .flags(vk::PipelineLayoutCreateFlags::empty())
            .set_layouts(&vk_set_layouts)
            .push_constant_ranges(&vk_push_constant_ranges);

        let handle = unsafe {
            self.shared
                .handle
                .create_pipeline_layout(&layout_info, None)
                .map_err(DeviceError::Other)?
        };

        let mut binding_arrays = BTreeMap::new();
        // for (group, &layout) in info.bind_group_layouts.iter().enumerate() {
        //
        // }

        Ok(PipelineLayout {
            handle,
            binding_arrays,
        })
    }

    pub fn create_raster_pipeline(
        &self,
        info: &RasterPipelineInfo<'_>,
    ) -> Result<RasterPipeline, DeviceError> {
        let dynamic_states = [
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            // vk::DynamicState::BLEND_CONSTANTS,
            // vk::DynamicState::STENCIL_REFERENCE,
        ];

        let mut stage_infos = Vec::new();
        let vertex_name = ffi::CString::new(info.vertex.entry_point).unwrap();
        // rust reference dies and rust compiler doesn't catch it
        let mut fragment_name = ffi::CString::new("").unwrap();

        stage_infos.push(
            vk::PipelineShaderStageCreateInfo::builder()
                .name(&vertex_name)
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(info.vertex.module.handle)
                .build(),
        );

        if let Some(fragment) = &info.fragment {
            fragment_name = ffi::CString::new(fragment.entry_point).unwrap();
            stage_infos.push(
                vk::PipelineShaderStageCreateInfo::builder()
                    .name(&fragment_name)
                    .stage(vk::ShaderStageFlags::FRAGMENT)
                    .module(fragment.module.handle)
                    .build(),
            );
        }

        let vertex_state = match &info.vertex_buffers {
            Some(buffers) => {
                let mut vertex_buffers = Vec::with_capacity(buffers.len());
                let mut vertex_attributes = Vec::new();

                for (i, vb) in buffers.iter().enumerate() {
                    vertex_buffers.push(vk::VertexInputBindingDescription {
                        binding: i as u32,
                        stride: vb.array_stride as u32,
                        input_rate: match vb.step_mode {
                            VertexStepMode::Vertex => vk::VertexInputRate::VERTEX,
                            VertexStepMode::Instance => vk::VertexInputRate::INSTANCE,
                        },
                    });
                    for at in vb.attributes {
                        vertex_attributes.push(vk::VertexInputAttributeDescription {
                            location: at.shader_location,
                            binding: i as u32,
                            format: conv::map_vertex_format(at.format),
                            offset: at.offset as u32,
                        });
                    }
                }
                vk::PipelineVertexInputStateCreateInfo::builder()
                    .vertex_binding_descriptions(&vertex_buffers)
                    .vertex_attribute_descriptions(&vertex_attributes)
                    .build()
            }
            None => vk::PipelineVertexInputStateCreateInfo::default()
        };

        let vk_input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(conv::map_topology(info.primitive.topology))
            .primitive_restart_enable(info.primitive.strip_index_format.is_some());

        let vk_sample_mask = [
            info.multisample.mask as u32,
            (info.multisample.mask >> 32) as u32,
        ];

        let vk_multisample = vk::PipelineMultisampleStateCreateInfo::builder()
            // .rasterization_samples(vk::SampleCountFlags::from_raw(info.multisample.count))
            // .alpha_to_coverage_enable(info.multisample.alpha_to_coverage_enabled)
            // .sample_mask(&vk_sample_mask);
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .alpha_to_coverage_enable(info.multisample.alpha_to_coverage_enabled)
            .min_sample_shading(1.0)
            .sample_mask(&[]);

        let mut vk_rasterization = vk::PipelineRasterizationStateCreateInfo::builder()
            .polygon_mode(conv::map_polygon_mode(info.primitive.polygon_mode))
            .front_face(conv::map_front_face(info.primitive.front_face))
            .line_width(info.primitive.line_width);

        if let Some(face_flags) = info.primitive.cull_mode {
            vk_rasterization = vk_rasterization.cull_mode(conv::map_cull_face(face_flags));
        }

        let mut vk_rasterization_conservative_state =
            vk::PipelineRasterizationConservativeStateCreateInfoEXT::builder()
                .conservative_rasterization_mode(vk::ConservativeRasterizationModeEXT::OVERESTIMATE)
                .build();

        if info.primitive.conservative {
            vk_rasterization = vk_rasterization.push_next(&mut vk_rasterization_conservative_state);
        }

        let mut vk_depth_clip_state =
            vk::PipelineRasterizationDepthClipStateCreateInfoEXT::builder()
                .depth_clip_enable(false)
                .build();

        if info.primitive.unclipped_depth {
            vk_rasterization = vk_rasterization.push_next(&mut vk_depth_clip_state);
        }

        let mut vk_depth_stencil = vk::PipelineDepthStencilStateCreateInfo::builder();

        // let mut vk_attachments = Vec::with_capacity(info.targets.len());
        let mut rendering_formats = Vec::with_capacity(info.targets.len());
        for target in info.targets {
            let vk_format: vk::Format = target.format.into();
            rendering_formats.push(vk_format);
        //     let mut vk_attachment = vk::PipelineColorBlendAttachmentState::builder()
        //         .color_write_mask(vk::ColorComponentFlags::from_raw(target.write_mask.bits()));
        //
        //     if let Some(ref blend) = target.blend {
        //         let (color_op, color_src, color_dst) = conv::map_blend_component(&blend.color);
        //         let (alpha_op, alpha_src, alpha_dst) = conv::map_blend_component(&blend.alpha);
        //         vk_attachment = vk_attachment
        //             .blend_enable(true)
        //             .color_blend_op(color_op)
        //             .src_color_blend_factor(color_src)
        //             .dst_color_blend_factor(color_dst)
        //             .alpha_blend_op(alpha_op)
        //             .src_alpha_blend_factor(alpha_src)
        //             .dst_alpha_blend_factor(alpha_dst);
        //     }
        //     vk_attachments.push(vk_attachment)
        }

        // let vk_color_blend =
        //     vk::PipelineColorBlendStateCreateInfo::builder()
        //         .attachments(&vk_attachments)
        //         .blend_constants([1.0, 1.0, 1.0, 1.0]);

        // let noop_stencil_state = vk::StencilOpState {
        //     fail_op: vk::StencilOp::KEEP,
        //     pass_op: vk::StencilOp::KEEP,
        //     depth_fail_op: vk::StencilOp::KEEP,
        //     compare_op: vk::CompareOp::ALWAYS,
        //     ..Default::default()
        // };
        //
        // let vk_depth_stencil = vk::PipelineDepthStencilStateCreateInfo {
        //     depth_test_enable: 1,
        //     depth_write_enable: 1,
        //     depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
        //     front: noop_stencil_state,
        //     back: noop_stencil_state,
        //     max_depth_bounds: 1.0,
        //     ..Default::default()
        // };

        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        }];

        let vk_color_blend = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

        let default_viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            min_depth: 0.0,
            max_depth: 0.0
        };

        let default_scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D { width: 1, height: 1 }
        };

        let vk_viewport = vk::PipelineViewportStateCreateInfo::builder()
            .flags(vk::PipelineViewportStateCreateFlags::empty())
            .viewports(std::slice::from_ref(&default_viewport))
            .scissors(std::slice::from_ref(&default_scissor))
            .scissor_count(1)
            .viewport_count(1);

        let vk_dynamic_state =
            vk::PipelineDynamicStateCreateInfo::builder()
                .dynamic_states(&dynamic_states);

        let mut pipeline_rendering_info = vk::PipelineRenderingCreateInfo::builder()
            .color_attachment_formats(&rendering_formats);

        let mut pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .push_next(&mut pipeline_rendering_info)
            .layout(info.layout.handle)
            .stages(&stage_infos)
            .vertex_input_state(&vertex_state)
            .input_assembly_state(&vk_input_assembly)
            .rasterization_state(&vk_rasterization)
            .viewport_state(&vk_viewport)
            .multisample_state(&vk_multisample)
            // .depth_stencil_state(&vk_depth_stencil)
            .color_blend_state(&vk_color_blend)
            .render_pass(vk::RenderPass::null())
            .dynamic_state(&vk_dynamic_state);

        let vk_infos = [pipeline_info.build()];

        let mut pipeline_handles = unsafe {
            self.shared
                .handle
                .create_graphics_pipelines(vk::PipelineCache::null(), &vk_infos, None)
                .map_err(|(p, e)| DeviceError::Other(e))?
        };

        let handle = pipeline_handles.pop().unwrap();

        Ok(RasterPipeline {
            device: self.shared.clone(),
            handle,
        })
    }
}
