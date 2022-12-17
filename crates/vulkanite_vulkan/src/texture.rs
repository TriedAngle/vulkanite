use crate::conv;
use crate::device::{Device, DeviceError};
use crate::pipeline::vt;
use ash::vk;
use gpu_alloc_ash::AshMemoryDevice;

#[derive(Debug)]
pub struct Texture {
    pub(crate) handle: vk::Image,
    pub(crate) usage: vt::TextureUsages,
    pub(crate) block: Option<gpu_alloc::MemoryBlock<vk::DeviceMemory>>,
}

#[derive(Debug, Copy, Clone)]
pub struct TextureView {
    pub(crate) handle: vk::ImageView,
}

impl Device {
    pub fn create_texture(&self, info: &vt::TextureInfo) -> Result<Texture, DeviceError> {
        let vk_info = vk::ImageCreateInfo::builder()
            .image_type(conv::map_texture_dimension(info.dimension))
            .format(conv::map_texture_format(info.format))
            .mip_levels(info.mip_levels)
            .array_layers(info.size.depth)
            .samples(vk::SampleCountFlags::from_raw(info.samples))
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(conv::map_texture_usages(info.usage))
            .extent(conv::map_extent3d(info.size))
            .sharing_mode(conv::map_sharing_mode(info.sharing))
            .initial_layout(vk::ImageLayout::UNDEFINED);

        let handle = unsafe {
            self.shared
                .handle
                .create_image(&vk_info, None)
                .map_err(DeviceError::Other)?
        };

        let requirements = unsafe { self.shared.handle.get_image_memory_requirements(handle) };

        let block = unsafe {
            self.allocator
                .lock()
                .alloc(
                    AshMemoryDevice::wrap(self.raw()),
                    gpu_alloc::Request {
                        size: requirements.size,
                        align_mask: requirements.alignment - 1,
                        usage: gpu_alloc::UsageFlags::FAST_DEVICE_ACCESS,
                        memory_types: requirements.memory_type_bits,
                    },
                )
                .unwrap()
        };

        unsafe {
            self.shared
                .handle
                .bind_image_memory(handle, *block.memory(), block.offset())
                .map_err(DeviceError::Other)?
        }

        Ok(Texture {
            handle,
            usage: info.usage,
            block: Some(block),
        })
    }

    pub fn free_texture(&self, texture: Texture) {
        if let Some(block) = texture.block {
            unsafe {
                self.allocator
                    .lock()
                    .dealloc(AshMemoryDevice::wrap(&self.shared.handle), block)
            }
        }
    }

    pub fn create_texture_view(
        &self,
        info: &vt::TextureViewInfo,
        texture: &Texture,
    ) -> Result<TextureView, DeviceError> {
        let vk_info = vk::ImageViewCreateInfo::builder()
            .flags(vk::ImageViewCreateFlags::empty())
            .image(texture.handle)
            .view_type(conv::map_texture_view_dimension(info.dimension))
            .format(conv::map_texture_format(info.format))
            .subresource_range(conv::map_texture_subresource_range(info.range));

        let handle = unsafe {
            self.shared
                .handle
                .create_image_view(&vk_info, None)
                .map_err(DeviceError::Other)?
        };

        Ok(TextureView { handle })
    }
}
