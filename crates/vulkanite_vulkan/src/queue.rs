use crate::device::{DeviceError, DeviceShared};
use crate::sync::{Fence, BinarySemaphore};
use ash::vk;
use parking_lot::Mutex;
use std::sync::Arc;
use tracing::info;
use crate::command::CommandBuffer;

#[derive(Debug)]
pub struct Queue {
    pub(crate) handle: Mutex<vk::Queue>,
    pub(crate) device: Arc<DeviceShared>,
    pub(crate) id: u64,
    pub(crate) family: u32,
    pub(crate) id_in_family: u32,
}

impl Queue {
    // pub fn wait(&self) {
    //     self.device.handle.wait
    // }
    pub fn submit(
        &mut self,
        command_buffers: impl IntoIterator<Item = CommandBuffer>,
        signal_binary_semaphores: &[&BinarySemaphore],
        // signal_timeline_semaphores: &[(&TimelineSemaphore, u64)],
        wait_binary_semaphores: &[&BinarySemaphore],
        // wait_timeline_semaphores: &[(&TimelineSemaphore, u64)],
        fence: Option<&Fence>,
    ) -> Result<(), DeviceError> {
        let vk_fence = match fence {
            Some(fence) => fence.handle,
            None => vk::Fence::null(),
        };

        let submit_command_buffers = command_buffers
            .into_iter()
            .map(|buffer| buffer.handle)
            .collect::<Vec<vk::CommandBuffer>>();

        let submit_signals = signal_binary_semaphores
            .iter()
            .map(|sema| sema.handle)
            .collect::<Vec<_>>();

        let mut submit_wait_masks = Vec::new();

        let submit_waits = wait_binary_semaphores
            .iter()
            .map(|sema| {
                submit_wait_masks.push(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT);
                sema.handle
            })
            .collect::<Vec<_>>();

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&submit_command_buffers)
            .wait_semaphores(&submit_waits)
            .wait_dst_stage_mask(&submit_wait_masks)
            .signal_semaphores(&submit_signals)
            .build();

        let handle = self.handle.lock();
        unsafe {
            self.device
                .handle
                .queue_submit(*handle, &[submit_info], vk_fence)
                .unwrap();
        }
        Ok(())
        // let mut submit_signals = Vec::new();
        // let mut submit_signals_values = Vec::new();
        // signal_timeline_semaphores.iter()
        //     .for_each(|(sema, value)| {
        //         submit_signals.push(sema.handle);
        //         submit_signals_values.push(*value)
        //     });
        //
        // signal_binary_semaphores.iter()
        //     .for_each(|sema| {
        //         submit_signals.push(sema.handle);
        //         submit_signals_values.push(0)
        //     });
        //
        // let mut submit_waits = Vec::new();
        // let mut submit_waits_values = Vec::new();
        // let mut submit_waits_masks = Vec::new();
        //
        // wait_timeline_semaphores.iter()
        //     .for_each(|(sema, value)| {
        //         submit_waits.push(sema.handle);
        //         submit_waits_values.push(*value);
        //         submit_waits_masks.push(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT);
        //     });
        //
        // wait_binary_semaphores.iter()
        //     .for_each(|sema| {
        //         submit_signals.push(sema.handle);
        //         submit_signals_values.push(0);
        //         submit_waits_masks.push(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT);
        //     });
        //
        // let timeline_info = vk::TimelineSemaphoreSubmitInfo::builder()
        //     .wait_semaphore_values(&submit_waits_values)
        //     .signal_semaphore_values(&submit_signals_values);
        //
        // let mut submit_info = vk::SubmitInfo::builder()
        //     .command_buffers(&vk_command_buffers)
        //     .wait_semaphores(&submit_waits)
        //     .signal_semaphores(&submit_signals)
        //     .wait_dst_stage_mask(&submit_waits_masks)
        //     .build();
        //
        //
        // submit_info.p_next = &timeline_info as *const _ as *const ffi::c_void;
        //
        // let handle = self.handle.lock();
        // unsafe { self.device.handle.queue_submit(*handle, &[submit_info], vk_fence).unwrap(); }
        // Ok(())
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct QueueCreateInfo<'q> {
    pub priorities: Vec<f32>,
    pub family: QueueFamily<'q>,
}

impl<'q> QueueCreateInfo<'q> {
    pub fn new(family: QueueFamily<'q>, priorities: Vec<f32>) -> Self {
        Self { priorities, family }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct QueueFamily<'q> {
    pub physical_device_id: u32,
    pub family_id: u32,
    pub properties: &'q vk::QueueFamilyProperties,
}

impl<'q> QueueFamily<'q> {
    pub fn supports_graphics(&self) -> bool {
        self.properties
            .queue_flags
            .contains(vk::QueueFlags::GRAPHICS)
    }

    pub fn supports_compute(&self) -> bool {
        self.properties
            .queue_flags
            .contains(vk::QueueFlags::COMPUTE)
    }

    pub fn supports_transfer(&self) -> bool {
        self.properties
            .queue_flags
            .contains(vk::QueueFlags::TRANSFER)
    }

    pub fn supports_sparse_binding(&self) -> bool {
        self.properties
            .queue_flags
            .contains(vk::QueueFlags::SPARSE_BINDING)
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        let device = self.device.handle.clone();
        unsafe {
            device.device_wait_idle().unwrap();
            info!("Destroyed: Queue");
        }
    }
}
