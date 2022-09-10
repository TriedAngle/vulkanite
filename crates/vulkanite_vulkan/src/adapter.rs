use crate::instance::InstanceShared;
use crate::queue::QueueFamily;
use crate::instance::Instance;
use ash::vk;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct Adapter {
    pub(crate) handle: vk::PhysicalDevice,
    pub(crate) instance: Arc<InstanceShared>,
    pub(crate) properties: vk::PhysicalDeviceProperties,
    pub(crate) features: vk::PhysicalDeviceFeatures,
    pub(crate) queue_families: Vec<vk::QueueFamilyProperties>,
}

impl Adapter {
    pub fn handle(&self) -> vk::PhysicalDevice {
        self.handle
    }

    pub fn instance_handle(&self) -> &ash::Instance {
        &self.instance.handle
    }

    pub fn queue_families<'a>(&self) -> impl Iterator<Item = QueueFamily<'_>> {
        self.queue_families
            .iter()
            .enumerate()
            .map(|(id, properties)| QueueFamily {
                physical_device_id: self.properties.device_id,
                family_id: id as u32,
                properties,
            })
    }
}

impl Instance {
    pub fn adapters(&self) -> impl Iterator<Item = Adapter> + '_ {
        let handle = &self.shared.handle;
        unsafe {
            let physical_devices = handle.enumerate_physical_devices().unwrap();
            physical_devices.into_iter().map(|p| {
                let p = p.clone();
                let properties = handle.get_physical_device_properties(p);
                let features = handle.get_physical_device_features(p);
                let queue_families = handle.get_physical_device_queue_family_properties(p);

                Adapter {
                    handle: p,
                    instance: self.shared.clone(),
                    properties,
                    features,
                    queue_families,
                }
            })
        }
    }
}

impl fmt::Debug for Adapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GPU: {{\n")?;
        write!(f, "  features: {:?},\n", self.features)?;
        write!(f, "  properties: {:?}\n", self.properties)?;
        write!(f, "  queue_families: {:?}\n", self.queue_families)?;
        write!(f, "}}")?;
        Ok(())
    }
}
