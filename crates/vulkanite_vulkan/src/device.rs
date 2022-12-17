use crate::adapter::Adapter;
use crate::command::CommandEncoder;
use crate::instance::InstanceShared;
use crate::queue::Queue;
use crate::queue::QueueCreateInfo;
use crate::types::{Extensions, Features};
use tracing::info;

use ash::vk;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::os::raw;
use std::sync::Arc;
use std::{error, ffi};

pub(crate) struct DeviceShared {
    pub(crate) handle: ash::Device,
    pub(crate) adapter: Arc<Adapter>,
    pub(crate) instance: Arc<InstanceShared>,
}

impl Debug for DeviceShared {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VkDevice {{ handle: {:?} }}", self.handle.handle())
    }
}

pub struct Device {
    pub(crate) shared: Arc<DeviceShared>,
    pub(crate) allocator: Mutex<gpu_alloc::GpuAllocator<vk::DeviceMemory>>,
    pub(crate) command_encoders: Mutex<HashMap<u64, CommandEncoder>>,
}

impl Device {
    pub fn raw(&self) -> &ash::Device {
        &self.shared.handle
    }
}

impl Adapter {
    pub fn request_device(
        &self,
        info: DeviceCreateInfo,
    ) -> Result<(Device, impl Iterator<Item = Queue>), DeviceError> {
        let DeviceCreateInfo {
            mut extensions,
            features,
            queue_families,
        } = info;

        let instance = self.instance.clone();
        if instance.render {
            extensions.vk_khr_swapchain = true;
            extensions.vk_khr_dynamic_rendering = true;
        }

        extensions.vk_khr_timeline_semaphore = true;

        let extension_names = Vec::<ffi::CString>::from(extensions);

        let layer_names = Vec::<ffi::CString>::from(instance.layers());

        let extension_pointers = extension_names
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<*const raw::c_char>>();

        let layer_pointers = layer_names
            .iter()
            .map(|layer| layer.as_ptr())
            .collect::<Vec<*const raw::c_char>>();

        let mut queues_to_get = Vec::with_capacity(queue_families.len());

        let queue_infos = queue_families
            .iter()
            .map(|family| {
                queues_to_get.extend((0..family.priorities.len() as u32).map(move |id| {
                    QueueToGet {
                        family: family.family.family_id,
                        id,
                    }
                }));
                vk::DeviceQueueCreateInfo {
                    flags: vk::DeviceQueueCreateFlags::empty(),
                    queue_family_index: family.family.family_id,
                    queue_count: family.priorities.len() as u32,
                    p_queue_priorities: family.priorities.as_ptr(),
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>();

        let features = vk::PhysicalDeviceFeatures::from(features);

        let device_info = vk::DeviceCreateInfo::builder()
            .flags(vk::DeviceCreateFlags::empty())
            .enabled_layer_names(&layer_pointers)
            .enabled_extension_names(&extension_pointers)
            .enabled_features(&features)
            .queue_create_infos(&queue_infos);

        let mut synchronization2 =
            vk::PhysicalDeviceSynchronization2Features::builder().synchronization2(true);

        let mut vulkan12features =
            vk::PhysicalDeviceVulkan12Features::builder().timeline_semaphore(true);

        let mut vulkan_dynamic_rendering =
            vk::PhysicalDeviceDynamicRenderingFeatures::builder().dynamic_rendering(true);

        let device_info = device_info
            .push_next(&mut vulkan_dynamic_rendering)
            .push_next(&mut vulkan12features)
            .push_next(&mut synchronization2)
            .build();

        let vk_handle_device = unsafe {
            instance
                .handle
                .create_device(self.handle, &device_info, None)
                .map_err(|err| DeviceError::Other(err))?
        };

        let vk_device = Arc::new(DeviceShared {
            handle: vk_handle_device,
            adapter: Arc::new(self.clone()),
            instance: instance.clone(),
        });

        let queue_iter = {
            use ash::vk::Handle;
            let device = vk_device.clone();
            queues_to_get
                .into_iter()
                .map(move |QueueToGet { family, id }| {
                    let vk_queue = unsafe { device.handle.get_device_queue(family, id) };
                    let queue_id = vk_queue.as_raw();
                    let queue = Queue {
                        handle: Mutex::new(vk_queue),
                        device: device.clone(),
                        id: queue_id,
                        family,
                        id_in_family: id,
                    };
                    queue
                })
        };

        let allocator = {
            let config = gpu_alloc::Config::i_am_prototyping();
            let properties = unsafe {
                gpu_alloc_ash::device_properties(
                    &self.instance.handle,
                    instance.version.to_vulkan(),
                    self.handle,
                )
                .unwrap()
            };

            gpu_alloc::GpuAllocator::new(config, properties)
        };

        let device = Device {
            shared: vk_device.clone(),
            allocator: Mutex::new(allocator),
            command_encoders: Mutex::new(HashMap::new()),
        };

        Ok((device, queue_iter))
    }
}

struct QueueToGet {
    family: u32,
    id: u32,
}

#[derive(Debug, Clone)]
pub struct DeviceCreateInfo<'q> {
    pub extensions: Extensions,
    pub features: Features,
    pub queue_families: Vec<QueueCreateInfo<'q>>,
}

impl<'q> Default for DeviceCreateInfo<'q> {
    fn default() -> Self {
        Self {
            extensions: Extensions::none(),
            features: Features::none(),
            queue_families: vec![],
        }
    }
}

#[derive(Debug)]
pub enum DeviceError {
    Other(vk::Result),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo!")?;
        Ok(())
    }
}

impl error::Error for DeviceError {}

impl Drop for DeviceShared {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_device(None);
        }
        info!("Destroyed: Device");
    }
}
