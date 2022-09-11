use crate::device::{Device, DeviceShared};
use crate::instance::{Instance, InstanceShared};
use crate::types::{PresentMode, Texture, TextureFormat, TextureUsages};
use tracing::{info, warn};

use crate::adapter::Adapter;
use crate::queue::Queue;
use crate::sync::{BinarySemaphore, Fence};
use ash::{extensions::khr, vk};
use raw_window_handle::HasRawWindowHandle;
use std::sync::Arc;
use std::{error, fmt};

#[derive(Clone)]
pub struct Surface {
    pub(crate) handle: vk::SurfaceKHR,
    pub(crate) loader: khr::Surface,
    pub(crate) instance: Arc<InstanceShared>,
    pub(crate) swapchain: Option<Swapchain>,
}

#[derive(Clone)]
pub struct Swapchain {
    pub(crate) handle: vk::SwapchainKHR,
    pub(crate) loader: khr::Swapchain,
    pub(crate) device: Arc<DeviceShared>,
    pub(crate) config: SurfaceConfig,
    pub(crate) images: Vec<vk::Image>,
    pub(crate) image_views: Vec<vk::ImageView>,
}

pub struct Frame {
    pub(crate) texture: Texture,
    pub(crate) view: vk::ImageView,
    pub(crate) suboptimal: bool,
    pub(crate) index: u32,
}

impl Surface {
    pub fn present_modes(
        &self,
        adapter: &Adapter,
    ) -> Result<impl Iterator<Item = PresentMode>, SurfaceError> {
        let modes = unsafe {
            self.loader
                .get_physical_device_surface_present_modes(adapter.handle, self.handle)
                .map_err(|err| SurfaceError::Other(err))?
        };

        Ok(modes
            .into_iter()
            .map(|mode| PresentMode::from(mode.as_raw())))
    }

    pub fn acquire_frame(
        &self,
        timeout_ms: u64,
        semaphore: Option<&BinarySemaphore>,
        fence: Option<&Fence>,
    ) -> Result<Option<Frame>, SurfaceError> {
        let sc = match &self.swapchain {
            None => {
                panic!("Surface not Configured")
            }
            Some(sc) => sc,
        };
        let timout = timeout_ms * 1_000_000;

        let vk_semaphore = match semaphore {
            Some(sema) => sema.handle,
            None => vk::Semaphore::null(),
        };
        let vk_fence = match fence {
            Some(fence) => fence.handle,
            None => vk::Fence::null(),
        };

        let (index, suboptimal) = unsafe {
            match sc
                .loader
                .acquire_next_image(sc.handle, timout, vk_semaphore, vk_fence)
            {
                Ok(res) => res,
                Err(err) => {
                    return match err {
                        vk::Result::TIMEOUT => Ok(None),
                        vk::Result::NOT_READY | vk::Result::ERROR_OUT_OF_DATE_KHR => {
                            Err(SurfaceError::Other(err))
                        }
                        _ => Err(SurfaceError::Other(err)),
                    }
                }
            }
        };

        let texture = Frame {
            texture: Texture {
                handle: sc.images[index as usize],
                allocation: None,
                usage: sc.config.usage,
            },
            view: sc.image_views[index as usize],
            suboptimal,
            index,
        };
        Ok(Some(texture))
    }

    pub fn formats(
        &self,
        adapter: &Adapter,
    ) -> Result<impl Iterator<Item = TextureFormat>, SurfaceError> {
        let formats = unsafe {
            self.loader
                .get_physical_device_surface_formats(adapter.handle, self.handle)
                .map_err(SurfaceError::Other)
        }?;
        Ok(formats
            .into_iter()
            .map(|format| TextureFormat::from(format.format.as_raw())))
    }

    pub fn capabilities(
        &self,
        adapter: &Adapter,
    ) -> Result<vk::SurfaceCapabilitiesKHR, SurfaceError> {
        unsafe {
            self.loader
                .get_physical_device_surface_capabilities(adapter.handle, self.handle)
                .map_err(SurfaceError::Other)
        }
    }

    pub fn configure(
        &mut self,
        device: &Device,
        config: &SurfaceConfig,
    ) -> Result<(), SurfaceError> {
        let old = self
            .swapchain
            .take()
            .map(|sc| unsafe { sc.release_resources(&device.shared.handle) });
        let swapchain = device.create_swapchain(self, config, old)?;
        self.swapchain = Some(swapchain);
        Ok(())
    }

    pub fn handle(&self) -> vk::SurfaceKHR {
        self.handle
    }

    pub fn swapchain(&self) -> &Swapchain {
        &self.swapchain.as_ref().unwrap()
    }
}

impl Swapchain {
    unsafe fn release_resources(self, device: &ash::Device) -> Self {
        {
            let _ = device.device_wait_idle();
        };
        self
    }

    pub fn handle(&self) -> vk::SwapchainKHR {
        self.handle
    }
}

impl Frame {
    pub fn present(
        &self,
        queue: &Queue,
        surface: &Surface,
        wait_binary_semaphores: &[&BinarySemaphore],
    ) -> Result<(), SurfaceError> {
        let sc = surface.swapchain.as_ref().unwrap();
        let scs = [sc.handle];
        let image_indices = [self.index];

        let semaphores = wait_binary_semaphores
            .iter()
            .map(|sema| sema.handle)
            .collect::<Vec<_>>();

        let present_info = vk::PresentInfoKHR::builder()
            .swapchains(&scs)
            .wait_semaphores(&semaphores)
            .image_indices(&image_indices);

        let suboptimal = {
            let handle = queue.handle.lock();
            unsafe {
                sc.loader
                    .queue_present(*handle, &present_info)
                    .map_err(|error| match error {
                        vk::Result::ERROR_OUT_OF_DATE_KHR => SurfaceError::Other(error),
                        _ => SurfaceError::Other(error),
                    })
            }
        };
        Ok(())
    }
}

impl Device {
    pub fn create_swapchain(
        &self,
        surface: &mut Surface,
        config: &SurfaceConfig,
        old_sc: Option<Swapchain>,
    ) -> Result<Swapchain, SurfaceError> {
        let loader = khr::Swapchain::new(&surface.instance.handle, &self.shared.handle);
        let device = self.shared.clone();

        let old = match old_sc {
            Some(osc) => osc.handle,
            None => vk::SwapchainKHR::null(),
        };

        let color_space = vk::ColorSpaceKHR::SRGB_NONLINEAR;
        let surface_capabilities = surface.capabilities(&self.shared.adapter)?;

        let surface_resolution = match surface_capabilities.current_extent.width {
            u32::MAX => vk::Extent2D {
                width: config.width,
                height: config.height,
            },
            _ => surface_capabilities.current_extent,
        };

        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let mode = match surface
            .present_modes(&self.shared.adapter)
            .unwrap()
            .find(|&m| m == config.mode)
        {
            Some(mode) => mode,
            None => {
                warn!(
                    "Present Mode: \"{:?}\" not present on Adapter => Fallback to \"Fifo\"",
                    config.mode
                );
                PresentMode::Fifo
            }
        };

        let sc_info = vk::SwapchainCreateInfoKHR::builder()
            .flags(vk::SwapchainCreateFlagsKHR::empty())
            .surface(surface.handle)
            .min_image_count(3)
            .image_format(config.format.into())
            .image_color_space(color_space)
            .image_extent(surface_resolution)
            .image_usage(config.usage.into())
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(mode.into())
            .clipped(true)
            .old_swapchain(old)
            .image_array_layers(1);

        let result =
            unsafe { loader.create_swapchain(&sc_info, None) }.map_err(SurfaceError::Other);

        if old != vk::SwapchainKHR::null() {
            unsafe { loader.destroy_swapchain(old, None) }
        }

        let handle = match result {
            Ok(handle) => handle,
            Err(e) => return Err(e),
        };

        let images = unsafe { loader.get_swapchain_images(handle) }.map_err(SurfaceError::Other)?;

        let image_views = images
            .iter()
            .map(|&image| {
                let create_info = vk::ImageViewCreateInfo::builder()
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(config.format.into())
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::R,
                        g: vk::ComponentSwizzle::G,
                        b: vk::ComponentSwizzle::B,
                        a: vk::ComponentSwizzle::A,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .image(image);
                unsafe { device.handle.create_image_view(&create_info, None).unwrap() }
            })
            .collect::<Vec<_>>();

        Ok(Swapchain {
            handle,
            loader,
            device,
            config: *config,
            images,
            image_views,
        })
    }
}

impl Instance {
    pub fn create_surface(
        &self,
        window_handle: &dyn HasRawWindowHandle,
    ) -> Result<Surface, SurfaceError> {
        let instance = self.shared.clone();
        let handle = unsafe {
            ash_window::create_surface(&instance.entry, &instance.handle, window_handle, None)
                .map_err(SurfaceError::Other)?
        };

        let loader = khr::Surface::new(&instance.entry, &instance.handle);

        Ok(Surface {
            handle,
            loader,
            instance,
            swapchain: None,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SurfaceConfig {
    pub usage: TextureUsages,
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub mode: PresentMode,
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            if let Some(sc) = &mut self.swapchain {
                {
                    let _ = sc.device.handle.device_wait_idle();
                }
                sc.loader.destroy_swapchain(sc.handle, None);
                info!("Destroyed: Swapchain")
            }
            self.loader.destroy_surface(self.handle, None);
            info!("Destroyed: Surface")
        }
    }
}

#[derive(Debug)]
pub enum SurfaceError {
    Other(vk::Result),
}

impl fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo!")?;
        Ok(())
    }
}

impl error::Error for SurfaceError {}
