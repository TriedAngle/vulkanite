use crate::device::{Device, DeviceError, DeviceShared};
use crate::surface::Frame;
use ash::vk;
use parking_lot::Mutex;
use std::sync::Arc;
use crate::queue::Queue;
use crate::types::{ImageTransitionLayout, RenderInfo};

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
    pub(crate) device: Arc<DeviceShared>,
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
        frame: &Frame,
    ) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }
        unsafe { handle.image_transition(old.into(), new.into(), frame.texture.handle) }
    }

    pub fn begin_rendering(&mut self, info: RenderInfo<'_>) {
        let mut handle = self.handle.lock();
        if handle.active == vk::CommandBuffer::null() {
            panic!("no active encoding");
        }

        let attachments = info
            .color_attachments
            .iter()
            .map(|attachment| attachment.to_vulkan(info.frame.view))
            .collect::<Vec<_>>();

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
            handle.begin_rendering(area, &attachments);
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
            device,
        })
    }
}

impl VkCommandEncoder {
    pub(crate) unsafe fn image_transition(
        &mut self,
        old: vk::ImageLayout,
        new: vk::ImageLayout,
        image: vk::Image,
    ) {
        let barrier = vk::ImageMemoryBarrier::builder()
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .old_layout(old)
            .new_layout(new)
            .image(image)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            )
            .build();

        let barriers = [barrier];
        self.device.handle.cmd_pipeline_barrier(
            self.active,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &barriers,
        );
    }

    pub(crate) unsafe fn begin_rendering(
        &mut self,
        area: vk::Rect2D,
        attachments: &[vk::RenderingAttachmentInfo],
    ) {
        let render_info = vk::RenderingInfo::builder()
            .render_area(area)
            .color_attachments(attachments)
            .layer_count(1);

        self.device
            .handle
            .cmd_begin_rendering(self.active, &render_info);
    }

    pub(crate) unsafe fn end_rendering(&mut self) {
        self.device.handle.cmd_end_rendering(self.active);
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
