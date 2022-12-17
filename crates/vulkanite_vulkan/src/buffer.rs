use crate::conv;
use crate::device::{Device, DeviceError};
use ash::vk;
use gpu_alloc_ash::AshMemoryDevice;
use parking_lot::Mutex;
use vulkanite_types as vt;

pub struct BufferInitInfo<'a> {
    pub label: Option<&'a str>,
    pub contents: &'a [u8],
    pub usage: vt::BufferUsages,
    pub sharing: vt::SharingMode,
}

pub struct Buffer {
    pub(crate) handle: vk::Buffer,
    pub(crate) block: Mutex<gpu_alloc::MemoryBlock<vk::DeviceMemory>>,
}

impl Device {
    pub fn create_buffer_init(&self, info: &BufferInitInfo<'_>) -> Result<Buffer, DeviceError> {
        let size = if info.contents.is_empty() {
            0 as vt::BufferAddress
        } else {
            let unpadded_size = info.contents.len() as vt::BufferAddress;

            let align_mask = vt::COPY_BUFFER_ALIGNMENT - 1;
            let padded_size =
                ((unpadded_size + align_mask) & !align_mask).max(vt::COPY_BUFFER_ALIGNMENT);
            padded_size
        };

        let vk_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(conv::map_buffer_usage(info.usage))
            .sharing_mode(conv::map_sharing_mode(info.sharing));

        let handle = unsafe {
            self.shared
                .handle
                .create_buffer(&vk_info, None)
                .map_err(DeviceError::Other)?
        };

        let requirements = unsafe { self.shared.handle.get_buffer_memory_requirements(handle) };

        let alloc_usage = if info
            .usage
            .intersects(vt::BufferUsages::MAP_READ | vt::BufferUsages::MAP_WRITE)
        {
            let mut flags = gpu_alloc::UsageFlags::HOST_ACCESS;
            flags.set(
                gpu_alloc::UsageFlags::DOWNLOAD,
                info.usage.contains(vt::BufferUsages::MAP_READ),
            );
            flags.set(
                gpu_alloc::UsageFlags::UPLOAD,
                info.usage.contains(vt::BufferUsages::MAP_WRITE),
            );
            flags
        } else {
            gpu_alloc::UsageFlags::FAST_DEVICE_ACCESS
        };

        let mut allocator = self.allocator.lock();
        let mut block = unsafe {
            allocator
                .alloc(
                    AshMemoryDevice::wrap(&self.shared.handle),
                    gpu_alloc::Request {
                        size: requirements.size,
                        align_mask: requirements.size - 1,
                        usage: alloc_usage,
                        memory_types: requirements.memory_type_bits,
                    },
                )
                .unwrap()
        };

        unsafe {
            block
                .write_bytes(AshMemoryDevice::wrap(&self.shared.handle), 0, info.contents)
                .unwrap();
        }
        unsafe {
            self.shared
                .handle
                .bind_buffer_memory(handle, *block.memory(), block.offset())
                .unwrap();
        }

        Ok(Buffer {
            handle,
            block: Mutex::new(block),
        })
    }

    pub fn free_buffer(&self, buffer: Buffer) {
        unsafe {
            self.shared.handle.destroy_buffer(buffer.handle, None);
            self.allocator.lock().dealloc(
                AshMemoryDevice::wrap(&self.shared.handle),
                buffer.block.into_inner(),
            )
        }
    }
}
