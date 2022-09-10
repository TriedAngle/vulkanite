use crate::device::{Device, DeviceError, DeviceShared};
use ash::vk;
use std::sync::Arc;
use std::{ffi, ptr};

#[derive(Debug, Clone)]
pub struct BinarySemaphore {
    pub(crate) device: Arc<DeviceShared>,
    pub(crate) handle: vk::Semaphore,
}

#[derive(Debug, Clone)]
pub struct TimelineSemaphore {
    pub(crate) device: Arc<DeviceShared>,
    pub(crate) handle: vk::Semaphore,
}

#[derive(Debug, Clone)]
pub struct Fence {
    pub(crate) device: Arc<DeviceShared>,
    pub(crate) handle: vk::Fence,
}

impl Fence {
    pub fn new(device: &Device) -> Self {
        let device = device.shared.clone();
        let info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

        let handle = unsafe { device.handle.create_fence(&info, None).unwrap() };
        Self { device, handle }
    }

    pub fn wait(&self, timeout_ms: u64) -> Result<(), DeviceError> {
        let timout = timeout_ms * 1_000_000;
        let fences = [self.handle];
        unsafe {
            self.device
                .handle
                .wait_for_fences(&fences, true, timout)
                .map_err(DeviceError::Other)
        }
    }

    pub fn reset(&self) -> Result<(), DeviceError> {
        let fences = [self.handle];
        unsafe {
            self.device
                .handle
                .reset_fences(&fences)
                .map_err(DeviceError::Other)
        }
    }

    pub fn wait_reset(&self, timeout_ms: u64) -> Result<(), DeviceError> {
        self.wait(timeout_ms)?;
        self.reset()?;
        Ok(())
    }
}

impl BinarySemaphore {
    pub fn new(device: &Device) -> Self {
        let device = device.shared.clone();
        let info = vk::SemaphoreCreateInfo::default();
        let handle = unsafe { device.handle.create_semaphore(&info, None).unwrap() };
        Self { device, handle }
    }
}

impl TimelineSemaphore {
    pub fn new(device: &Device, initial_value: u64) -> Self {
        let device = device.shared.clone();
        let timeline_info = vk::SemaphoreTypeCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_TYPE_CREATE_INFO,
            p_next: ptr::null(),
            semaphore_type: vk::SemaphoreType::TIMELINE,
            initial_value,
        };

        let info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: (&timeline_info) as *const _ as *const ffi::c_void,
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let handle = unsafe { device.handle.create_semaphore(&info, None).unwrap() };
        Self { device, handle }
    }

    pub fn set_value(&self, value: u64) -> Result<(), DeviceError> {
        let info = vk::SemaphoreSignalInfo::builder()
            .semaphore(self.handle)
            .value(value);

        unsafe {
            self.device
                .handle
                .signal_semaphore(&info)
                .map_err(|err| DeviceError::Other(err))
        }
    }

    pub fn value(&self) -> Result<u64, DeviceError> {
        unsafe {
            self.device
                .handle
                .get_semaphore_counter_value(self.handle)
                .map_err(|err| DeviceError::Other(err))
        }
    }

    pub fn wait_for_value(&self, value: u64, timout_ms: u64) -> Result<(), DeviceError> {
        let semaphores = &[self.handle];
        let values = &[value];

        let info = vk::SemaphoreWaitInfo::builder()
            .semaphores(semaphores)
            .values(values);

        unsafe {
            self.device
                .handle
                .wait_semaphores(&info, timout_ms)
                .map_err(DeviceError::Other)
        }
    }
}

impl Device {
    pub fn create_binary_semaphore(&self) -> BinarySemaphore {
        BinarySemaphore::new(&self)
    }

    pub fn create_timeline_semaphore(&self, initial_value: u64) -> TimelineSemaphore {
        TimelineSemaphore::new(&self, initial_value)
    }

    pub fn create_fence(&self) -> Fence {
        Fence::new(&self)
    }
}

impl Drop for BinarySemaphore {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_semaphore(self.handle, None) }
    }
}

impl Drop for TimelineSemaphore {
    fn drop(&mut self) {
        unsafe { self.device.handle.destroy_semaphore(self.handle, None) }
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            self.device
                .handle
                .wait_for_fences(&[self.handle], true, 100000000)
                .unwrap();
            self.device.handle.destroy_fence(self.handle, None)
        }
    }
}
