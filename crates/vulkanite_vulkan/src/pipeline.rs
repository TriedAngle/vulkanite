use crate::conv;
use crate::device::{Device, DeviceError, DeviceShared};
use crate::shader::ShaderModule;
use ash::vk;
use std::collections::BTreeMap;
use std::ffi;
use std::num::NonZeroU32;
use std::sync::Arc;
pub use vulkanite_types as vt;

pub struct PipelineLayout {
    pub(crate) handle: vk::PipelineLayout,
    pub(crate) binding_arrays: naga::back::spv::BindingMap,
}

#[derive(Debug)]
pub struct BindGroupLayout {
    handle: vk::DescriptorSetLayout,
}

pub struct PipelineLayoutInfo<'a> {
    pub flags: vt::PipelineLayoutFlags,
    pub bind_group_layouts: &'a [&'a BindGroupLayout],
    pub push_constant_ranges: &'a [vt::PushConstantRange],
}

pub struct BindGroupLayoutDescriptor<'a> {
    entries: &'a [BindGroupLayoutEntry],
}

pub struct BindGroupLayoutEntry {
    binding: u32,
    visibility: vt::ShaderStages,
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

pub struct RasterPipelineInfo<'a> {
    pub layout: &'a PipelineLayout,
    pub vertex: ShaderStage<'a>,
    pub vertex_buffers: &'a [vt::VertexBufferLayout<'a>],
    pub fragment: Option<ShaderStage<'a>>,
    pub primitive: vt::PrimitiveState,
    pub depth_stencil: Option<vt::DepthStencilState>,
    pub multisample: vt::MultisampleState,
    pub targets: &'a [vt::ColorTargetState],
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

        let binding_arrays = BTreeMap::new();
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
            vk::DynamicState::BLEND_CONSTANTS,
            vk::DynamicState::STENCIL_REFERENCE,
        ];

        let vertex_name = ffi::CString::new(info.vertex.entry_point).unwrap();
        // rust reference dies and rust compiler doesn't catch it
        #[allow(unused_assignments)] // idk why rust forces me to do this lmao
        let mut fragment_name = ffi::CString::new("").unwrap();

        let mut stages = Vec::new();
        let mut vertex_buffers = Vec::with_capacity(info.vertex_buffers.len());
        let mut vertex_attributes = Vec::new();

        for (i, vb) in info.vertex_buffers.iter().enumerate() {
            vertex_buffers.push(vk::VertexInputBindingDescription {
                binding: i as u32,
                stride: vb.array_stride as u32,
                input_rate: match vb.step_mode {
                    vt::VertexStepMode::Vertex => vk::VertexInputRate::VERTEX,
                    vt::VertexStepMode::Instance => vk::VertexInputRate::INSTANCE,
                },
            });
            for at in vb.attributes {
                vertex_attributes.push(vk::VertexInputAttributeDescription {
                    location: at.location,
                    binding: i as u32,
                    format: conv::map_vertex_format(at.format),
                    offset: at.offset as u32,
                });
            }
        }

        let vk_vertex_input = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&vertex_buffers)
            .vertex_attribute_descriptions(&vertex_attributes)
            .build();

        stages.push(
            vk::PipelineShaderStageCreateInfo::builder()
                .name(&vertex_name)
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(info.vertex.module.handle)
                .build(),
        );

        if let Some(fragment) = &info.fragment {
            fragment_name = ffi::CString::new(fragment.entry_point).unwrap();
            stages.push(
                vk::PipelineShaderStageCreateInfo::builder()
                    .name(&fragment_name)
                    .stage(vk::ShaderStageFlags::FRAGMENT)
                    .module(fragment.module.handle)
                    .build(),
            );
        }

        let vk_input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(conv::map_topology(info.primitive.topology))
            .primitive_restart_enable(info.primitive.strip_index_format.is_some())
            .build();

        let vk_sample_mask = [
            info.multisample.mask as u32,
            (info.multisample.mask >> 32) as u32,
        ];

        let vk_multisample = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::from_raw(info.multisample.count))
            .alpha_to_coverage_enable(info.multisample.alpha_to_coverage_enabled)
            .sample_mask(&vk_sample_mask);

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

        if let Some(depth_stencil) = &info.depth_stencil {
            vk_rasterization = vk_rasterization
                .depth_bias_enable(true)
                .depth_bias_constant_factor(depth_stencil.bias.constant)
                .depth_bias_clamp(depth_stencil.bias.clamp)
                .depth_bias_slope_factor(depth_stencil.bias.slope);

            vk_depth_stencil = vk_depth_stencil
                .depth_test_enable(true)
                .depth_write_enable(depth_stencil.write)
                .depth_compare_op(conv::map_depth_function(depth_stencil.depth_compare))
                .depth_bounds_test_enable(false)
                .stencil_test_enable(false)
                .min_depth_bounds(0.0)
                .max_depth_bounds(1.0);
        }

        let mut vk_attachments = Vec::with_capacity(info.targets.len());
        let mut rendering_formats = Vec::with_capacity(info.targets.len());
        for target in info.targets {
            let vk_format = conv::map_texture_format(target.format);
            rendering_formats.push(vk_format);
            let mut vk_attachment = vk::PipelineColorBlendAttachmentState::builder()
                .color_write_mask(vk::ColorComponentFlags::from_raw(target.write_mask.bits()));

            if let Some(ref blend) = target.blend {
                let (color_op, color_src, color_dst) = conv::map_blend_component(&blend.color);
                let (alpha_op, alpha_src, alpha_dst) = conv::map_blend_component(&blend.alpha);
                vk_attachment = vk_attachment
                    .blend_enable(true)
                    .color_blend_op(color_op)
                    .src_color_blend_factor(color_src)
                    .dst_color_blend_factor(color_dst)
                    .alpha_blend_op(alpha_op)
                    .src_alpha_blend_factor(alpha_src)
                    .dst_alpha_blend_factor(alpha_dst);
            }
            vk_attachments.push(vk_attachment.build())
        }

        let vk_color_blend =
            vk::PipelineColorBlendStateCreateInfo::builder().attachments(&vk_attachments);

        let default_viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            min_depth: 0.0,
            max_depth: 0.0,
        };

        let default_scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width: 1,
                height: 1,
            },
        };

        let vk_viewport = vk::PipelineViewportStateCreateInfo::builder()
            .flags(vk::PipelineViewportStateCreateFlags::empty())
            .viewports(std::slice::from_ref(&default_viewport))
            .scissors(std::slice::from_ref(&default_scissor));

        let vk_dynamic_state =
            vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_states);

        let mut pipeline_rendering_info =
            vk::PipelineRenderingCreateInfo::builder().color_attachment_formats(&rendering_formats);

        if let Some(depth) = &info.depth_stencil {
            pipeline_rendering_info = pipeline_rendering_info
                .depth_attachment_format(conv::map_texture_format(depth.format));
        }

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .layout(info.layout.handle)
            .stages(&stages)
            .vertex_input_state(&vk_vertex_input)
            .input_assembly_state(&vk_input_assembly)
            .rasterization_state(&vk_rasterization)
            .viewport_state(&vk_viewport)
            .multisample_state(&vk_multisample)
            .depth_stencil_state(&vk_depth_stencil)
            .color_blend_state(&vk_color_blend)
            .render_pass(vk::RenderPass::null())
            .dynamic_state(&vk_dynamic_state)
            .push_next(&mut pipeline_rendering_info);

        let vk_infos = [pipeline_info.build()];

        let mut pipeline_handles = unsafe {
            self.shared
                .handle
                .create_graphics_pipelines(vk::PipelineCache::null(), &vk_infos, None)
                .map_err(|(_p, e)| DeviceError::Other(e))?
        };

        let handle = pipeline_handles.pop().unwrap();

        Ok(RasterPipeline {
            device: self.shared.clone(),
            handle,
        })
    }
}
