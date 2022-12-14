use crate::buffer::Buffer;
use crate::conv;
use crate::conv::map_depth_attachment_info;
use crate::device::{Device, DeviceError, DeviceShared};
use crate::pipeline::{vt, PipelineLayout, RasterPipeline};
use crate::queue::Queue;
use crate::surface::Frame;
use crate::types::{ImageTransitionLayout, RenderInfo};
use ash::vk;
use parking_lot::Mutex;
use std::ops::Range;
use std::sync::Arc;
pub use vulkanite_types::{AccessFlags, StageFlags};

const BUFFER_COUNT: u32 = 8;

#[derive(Debug)]
pub(crate) struct VkCommandEncoder {
    pub(crate) handle: Mutex<vk::CommandPool>,
    pub(crate) device: Arc<DeviceShared>,
    pub(crate) active: vk::CommandBuffer,
    pub(crate) primary: Vec<vk::CommandBuffer>,
    pub(crate) secondary: Vec<vk::CommandBuffer>,
}

#[derive(Debug, Clone)]
pub struct CommandEncoder {
    pub(crate) handle: Arc<Mutex<VkCommandEncoder>>,
}

impl CommandEncoder {
    pub fn finish(&mut self) -> CommandBuffer {
        let buffer = self.end_encoding();

        buffer
    }

    pub fn begin_encoding(&mut self) {
        unsafe { self.handle.lock().begin_encoding().unwrap() }
    }

    fn end_encoding(&mut self) -> CommandBuffer {
        if self.handle.lock().active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }
        unsafe { self.handle.lock().end_encoding().unwrap() }
    }

    pub fn frame_transition(
        &mut self,
        old: ImageTransitionLayout,
        new: ImageTransitionLayout,
        src_stage: Option<StageFlags>,
        src_access: Option<AccessFlags>,
        dst_stage: Option<StageFlags>,
        dst_access: Option<AccessFlags>,
        frame: &Frame,
    ) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }
        unsafe {
            handle.image_transition(
                old.into(),
                new.into(),
                src_stage,
                src_access,
                dst_stage,
                dst_access,
                frame.texture.handle,
            )
        }
    }

    pub fn begin_rendering(&mut self, info: RenderInfo<'_>) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }

        let attachments = info
            .color_attachments
            .iter()
            .map(|attachment| conv::map_render_attachment_info(attachment))
            .collect::<Vec<_>>();

        let depth = info
            .depth_attachment
            .map(|depth| map_depth_attachment_info(&depth));

        let area = vk::Rect2D {
            offset: vk::Offset2D {
                x: info.offset.0,
                y: info.offset.1,
            },
            extent: vk::Extent2D {
                width: info.area.0,
                height: info.area.1,
            },
        };

        unsafe {
            handle.begin_rendering(area, &attachments, depth, None);
        }
    }

    pub fn end_rendering(&mut self) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }
        unsafe {
            handle.end_rendering();
        }
    }

    pub fn bind_raster_pipeline(&mut self, pipeline: &RasterPipeline) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }
        unsafe {
            handle.bind_raster_pipeline(pipeline);
        }
    }

    pub fn draw(&mut self, vertex: Range<u32>, instance: Range<u32>) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }
        let vertex_count = vertex.len() as u32;
        let instance_count = instance.len() as u32;
        unsafe {
            handle.draw(vertex.start, vertex_count, instance.start, instance_count);
        }
    }

    pub fn bind_vertex_buffer(&mut self, index: u32, buffer: &Buffer) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }

        unsafe {
            handle.bind_vertex_buffer(index, buffer);
        }
    }

    pub fn push_constants(
        &mut self,
        layout: &PipelineLayout,
        stages: vt::ShaderStages,
        offset: u32,
        data: &[u8],
    ) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }
        unsafe {
            handle.push_constants(layout.handle, conv::map_shader_stage(stages), offset, data);
        }
    }
}

impl VkCommandEncoder {
    pub(crate) unsafe fn image_transition(
        &mut self,
        old: vk::ImageLayout,
        new: vk::ImageLayout,
        src_stage: Option<StageFlags>,
        src_access: Option<AccessFlags>,
        dst_stage: Option<StageFlags>,
        dst_access: Option<AccessFlags>,
        image: vk::Image,
    ) {
        // let mut barrier = vk::ImageMemoryBarrier2::builder()
        //     .old_layout(old)
        //     .new_layout(new)
        //     .image(image)
        //     .subresource_range(
        //         vk::ImageSubresourceRange::builder()
        //             .aspect_mask(vk::ImageAspectFlags::COLOR)
        //             .base_mip_level(0)
        //             .level_count(1)
        //             .base_array_layer(0)
        //             .layer_count(1)
        //             .build()
        //     );

        let mut barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(old.into())
            .new_layout(new.into())
            .image(image)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            );

        let mut src_stage_mask = vk::PipelineStageFlags::empty();
        let mut dst_stage_mask = vk::PipelineStageFlags::empty();
        let dependency_flags = vk::DependencyFlags::empty();

        if let Some(mask) = src_stage {
            // barrier = barrier.src_stage_mask(vk::PipelineStageFlags::from_raw(mask.bits() as vk::Flags))
            src_stage_mask = vk::PipelineStageFlags::from_raw(mask.bits());
        }
        if let Some(mask) = dst_stage {
            // barrier = barrier.src_stage_mask(vk::PipelineStageFlags::from_raw(mask.bits() as vk::Flags))
            dst_stage_mask = vk::PipelineStageFlags::from_raw(mask.bits());
        }

        if let Some(mask) = src_access {
            barrier = barrier.src_access_mask(vk::AccessFlags::from_raw(mask.bits() as vk::Flags))
        }
        if let Some(mask) = dst_access {
            barrier = barrier.dst_access_mask(vk::AccessFlags::from_raw(mask.bits() as vk::Flags))
        }

        let barriers = [barrier.build()];
        //
        // let dependency_info = vk::DependencyInfo::builder()
        //     .dependency_flags(vk::DependencyFlags::empty())
        //     .image_memory_barriers(&barriers);
        //
        //
        // self.device.handle.cmd_pipeline_barrier2(
        //     self.active,
        //     &dependency_info,
        // );

        self.device.handle.cmd_pipeline_barrier(
            self.active,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            &[],
            &[],
            &barriers,
        )
    }

    pub(crate) unsafe fn begin_rendering(
        &mut self,
        area: vk::Rect2D,
        attachments: &[vk::RenderingAttachmentInfo],
        depth_attachment: Option<vk::RenderingAttachmentInfo>,
        stencil_attachment: Option<vk::RenderingAttachmentInfo>,
    ) {
        let mut render_info = vk::RenderingInfo::builder()
            .render_area(area)
            .color_attachments(attachments)
            .view_mask(0)
            .layer_count(1);

        if let Some(depth) = &depth_attachment {
            render_info = render_info.depth_attachment(depth);
        }

        if let Some(stencil) = &stencil_attachment {
            render_info = render_info.stencil_attachment(stencil);
        }

        let viewports = [vk::Viewport::builder()
            .width(area.extent.width as f32)
            .height(area.extent.height as f32)
            .x(area.offset.x as f32)
            .y(area.offset.y as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build()];

        self.device.handle.cmd_set_scissor(self.active, 0, &[area]);
        self.device
            .handle
            .cmd_set_viewport(self.active, 0, &viewports);

        self.device
            .handle
            .cmd_begin_rendering(self.active, &render_info);
    }

    pub(crate) unsafe fn end_rendering(&mut self) {
        self.device.handle.cmd_end_rendering(self.active);
    }

    pub(crate) unsafe fn bind_vertex_buffer(&mut self, index: u32, buffer: &Buffer) {
        let vk_buffers = [buffer.handle];
        let vk_offsets = [buffer.block.lock().offset()];
        self.device
            .handle
            .cmd_bind_vertex_buffers(self.active, index, &vk_buffers, &vk_offsets)
    }

    pub(crate) unsafe fn push_constants(
        &mut self,
        layout: vk::PipelineLayout,
        stages: vk::ShaderStageFlags,
        offset: u32,
        data: &[u8],
    ) {
        self.device
            .handle
            .cmd_push_constants(self.active, layout, stages, offset, data)
    }

    pub(crate) unsafe fn begin_encoding(&mut self) -> Result<(), DeviceError> {
        if self.primary.is_empty() {
            self.allocate(BUFFER_COUNT, false)?
        }

        let active = self.primary.pop().unwrap();

        let command_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        self.device
            .handle
            .begin_command_buffer(active, &command_begin_info)
            .map_err(DeviceError::Other)?;

        self.active = active;

        Ok(())
    }

    pub(crate) unsafe fn end_encoding(&mut self) -> Result<CommandBuffer, DeviceError> {
        let active = self.active;
        self.active = vk::CommandBuffer::null();

        self.device
            .handle
            .end_command_buffer(active)
            .map_err(DeviceError::Other)?;

        Ok(CommandBuffer { handle: active })
    }

    pub(crate) unsafe fn bind_raster_pipeline(&mut self, pipeline: &RasterPipeline) {
        self.device.handle.cmd_bind_pipeline(
            self.active,
            vk::PipelineBindPoint::GRAPHICS,
            pipeline.handle,
        )
    }

    pub(crate) unsafe fn draw(
        &mut self,
        start_vertex: u32,
        count_vertex: u32,
        start_instance: u32,
        count_instance: u32,
    ) {
        self.device.handle.cmd_draw(
            self.active,
            count_vertex,
            count_instance,
            start_vertex,
            start_instance,
        )
    }

    unsafe fn allocate(&mut self, count: u32, secondary: bool) -> Result<(), DeviceError> {
        let buffer_info = {
            let handle = self.handle.lock();
            vk::CommandBufferAllocateInfo::builder()
                .command_pool(*handle)
                .command_buffer_count(count)
                .level(if secondary {
                    vk::CommandBufferLevel::SECONDARY
                } else {
                    vk::CommandBufferLevel::PRIMARY
                })
        };

        let buffers = self
            .device
            .handle
            .allocate_command_buffers(&buffer_info)
            .map_err(DeviceError::Other)?;

        if secondary {
            self.primary.extend(buffers);
        } else {
            self.primary.extend(buffers);
        }

        Ok(())
    }
}

impl Device {
    pub fn command_encoder(&self, info: CommandEncoderInfo<'_>) -> CommandEncoder {
        let mut encoders = self.command_encoders.lock();
        let id = info.queue.id();

        if encoders.contains_key(&id) {
            return encoders.get(&id).unwrap().clone();
        }

        let encoder = self.allocate_command_encoder(info).unwrap();
        encoders.insert(id, encoder);
        encoders.get(&id).unwrap().clone()
    }

    fn allocate_command_encoder(
        &self,
        info: CommandEncoderInfo<'_>,
    ) -> Result<CommandEncoder, DeviceError> {
        let CommandEncoderInfo { queue } = info;

        let command_pool_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue.family);

        let handle = unsafe {
            self.shared
                .handle
                .create_command_pool(&command_pool_info, None)
                .map_err(DeviceError::Other)?
        };

        let device = self.shared.clone();

        let vk_command_encoder = VkCommandEncoder {
            handle: Mutex::new(handle),
            device: device.clone(),
            active: vk::CommandBuffer::null(),
            primary: vec![],
            secondary: vec![],
        };

        Ok(CommandEncoder {
            handle: Arc::new(Mutex::new(vk_command_encoder)),
        })
    }
}

pub struct CommandBuffer {
    pub(crate) handle: vk::CommandBuffer,
}

pub struct CommandEncoderInfo<'q> {
    pub queue: &'q Queue,
}

impl Drop for VkCommandEncoder {
    fn drop(&mut self) {
        let handle = self.handle.lock();
        unsafe {
            let _ = self.device.handle.device_wait_idle();
            // vulkan errors when buffer amount is 0, so let's just check that nothing is empty lmao
            if self.active != vk::CommandBuffer::null() {
                self.device
                    .handle
                    .free_command_buffers(*handle, &[self.active]);
            }
            if !self.primary.is_empty() {
                self.device
                    .handle
                    .free_command_buffers(*handle, &self.primary);
            }
            if !self.secondary.is_empty() {
                self.device
                    .handle
                    .free_command_buffers(*handle, &self.secondary);
            }
            self.device.handle.destroy_command_pool(*handle, None);
        }
    }
}
