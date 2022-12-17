use crate::debug::{VkDebug, VkDebugCallback};
use crate::types::{Extensions, Layers};
use crate::utils::Version;
use ash::vk;
use raw_window_handle::HasRawDisplayHandle;
use std::fmt::{Debug, Display, Formatter};
use std::os::raw;
use std::sync::Arc;
use std::{error, ffi, ptr};
use tracing::info;

pub(crate) struct InstanceShared {
    pub(crate) entry: ash::Entry,
    pub(crate) debug: Option<VkDebug>,
    pub(crate) handle: ash::Instance,
    pub(crate) extensions: Extensions,
    pub(crate) version: Version,
    pub(crate) render: bool,
    pub(crate) layers: Layers,
}

pub struct Instance {
    pub(crate) shared: Arc<InstanceShared>,
}

impl Instance {
    pub fn new(info: InstanceCreateInfo) -> Result<Self, InstanceCreationError> {
        let instance =
            InstanceShared::init(info).map_err(|err| InstanceCreationError::Other(err))?;

        Ok(Self {
            shared: Arc::new(instance),
        })
    }

    pub fn handle(&self) -> &ash::Instance {
        &self.shared.handle
    }
}

impl InstanceShared {
    pub fn init(info: InstanceCreateInfo) -> Result<Self, vk::Result> {
        let InstanceCreateInfo {
            application_name,
            application_version,
            engine_name,
            engine_version,
            layers,
            mut extensions,
            vulkan_version,
            render,
            window,
            debug,
        } = info;

        #[cfg(not(target_os = "macos"))]
        let entry = ash::Entry::linked();
        #[cfg(target_os = "macos")]
        let entry = ash_molten::load();

        let app_name_c = application_name.map(|name| ffi::CString::new(name.as_bytes()).unwrap());

        let engine_name_c = engine_name.map(|name| ffi::CString::new(name).unwrap());

        let layer_names = Vec::<ffi::CString>::from(layers);

        // some defaults
        extensions.vk_khr_get_physical_device_properties2 = true;

        if render {
            extensions.vk_khr_surface = true;
            if window.is_none() {
                if cfg!(all(
                    unix,
                    not(target_os = "android"),
                    not(target_os = "macos")
                )) {
                    extensions.vk_khr_xlib_surface = true;
                    extensions.vk_khr_xcb_surface = true;
                    extensions.vk_khr_wayland_surface = true;
                }
                if cfg!(target_os = "android") {
                    extensions.vk_khr_android_surface = true;
                }
                if cfg!(target_os = "windows") {
                    extensions.vk_khr_win32_surface = true;
                }
                if cfg!(target_os = "macos") {
                    extensions.vk_ext_metal_surface = true;
                }
            }
        }

        if debug {
            extensions.vk_ext_debug_utils = true;
        }

        let layer_pointers = layer_names
            .iter()
            .map(|layer| layer.as_ptr())
            .collect::<Vec<*const raw::c_char>>();

        let window_extensions = if let Some(handle) = window {
            ash_window::enumerate_required_extensions(handle.raw_display_handle()).unwrap()
        } else {
            &[]
        };

        let extension_names = Vec::<ffi::CString>::from(extensions);

        info!("Instance Layers: {:?}", layer_names);
        info!("Instance Extensions: {:?}", extension_names);

        let mut extension_pointers = extension_names
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<*const raw::c_char>>();

        for extension in window_extensions {
            extension_pointers.push(*extension)
        }

        let version = application_version;

        let app_info = vk::ApplicationInfo {
            p_application_name: app_name_c
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(ptr::null()),
            application_version: version.to_vulkan(),
            p_engine_name: engine_name_c
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(ptr::null()),
            engine_version: engine_version.to_vulkan(),
            api_version: vulkan_version.to_vulkan(),
            ..Default::default()
        };

        let instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layer_pointers)
            .enabled_extension_names(&extension_pointers);

        let handle = unsafe { entry.create_instance(&instance_info, None)? };

        let debug = if debug {
            Some(VkDebug::init(&entry, &handle, VkDebugCallback::default())?)
        } else {
            None
        };

        Ok(Self {
            entry,
            handle,
            extensions,
            debug,
            version,
            layers,
            render,
        })
    }

    pub fn extensions(&self) -> Extensions {
        self.extensions
    }

    pub fn layers(&self) -> Layers {
        self.layers
    }
}

#[derive(Clone)]
pub struct InstanceCreateInfo<'a> {
    pub application_name: Option<String>,
    pub application_version: Version,
    pub engine_name: Option<String>,
    pub engine_version: Version,
    pub layers: Layers,
    pub extensions: Extensions,
    pub vulkan_version: Version,
    pub render: bool,
    pub window: Option<&'a dyn HasRawDisplayHandle>,
    pub debug: bool,
}

impl Default for InstanceCreateInfo<'_> {
    fn default() -> Self {
        Self {
            application_name: None,
            application_version: Default::default(),
            engine_name: None,
            engine_version: Default::default(),
            layers: Layers::default(),
            extensions: Extensions::none(),
            vulkan_version: Version::V1_3,
            render: true,
            window: None,
            debug: true,
        }
    }
}

#[derive(Debug)]
pub enum InstanceCreationError {
    Other(vk::Result),
}

impl Display for InstanceCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo!")?;
        Ok(())
    }
}

impl error::Error for InstanceCreationError {}

impl Drop for InstanceShared {
    fn drop(&mut self) {
        unsafe {
            if let Some(debug) = self.debug.take() {
                debug
                    .loader
                    .destroy_debug_utils_messenger(debug.messenger, None);

                info!("Destroyed: Debug");
            }
            self.handle.destroy_instance(None);
            info!("Destroyed: Instance");
        }
    }
}
