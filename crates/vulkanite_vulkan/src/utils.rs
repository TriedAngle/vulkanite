use ash::vk;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Version {
    pub variant: u32,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub const V1_0: Self = Self::new(0, 1, 0, 0);
    pub const V1_1: Self = Self::new(0, 1, 1, 0);
    pub const V1_2: Self = Self::new(0, 1, 2, 0);
    pub const V1_3: Self = Self::new(0, 1, 3, 0);

    pub const fn new(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
        Self {
            variant,
            major,
            minor,
            patch,
        }
    }

    pub const fn from_vulkan(vk_version: u32) -> Self {
        let variant = vk::api_version_variant(vk_version);
        let major = vk::api_version_major(vk_version);
        let minor = vk::api_version_minor(vk_version);
        let patch = vk::api_version_patch(vk_version);
        Self {
            variant,
            major,
            minor,
            patch,
        }
    }

    pub const fn to_vulkan(&self) -> u32 {
        vk::make_api_version(self.variant, self.major, self.minor, self.patch)
    }
}
