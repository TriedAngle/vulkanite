use ash::vk;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextureFormatColorSpace {
    SrgbNonLinear = 0,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceDisplayP3NonlinearExt = 1000104001,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceExtendedSrgbLinearExt = 1000104002,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceDisplayP3LinearExt = 1000104003,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceDciP3NonlinearExt = 1000104004,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceBt709LinearExt = 1000104005,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceBt709NonlinearExt = 1000104006,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceBt2020LinearExt = 1000104007,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceHdr10St2084Ext = 1000104008,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceDolbyvisionExt = 1000104009,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceHdr10HlgExt = 1000104010,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceAdobergbLinearExt = 1000104011,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceAdobergbNonlinearExt = 1000104012,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpacePassThroughExt = 1000104013,
    // Provided by VK_EXT_swapchain_colorspace
    VkColorSpaceExtendedSrgbNonlinearExt = 1000104014,
    // Provided by VK_AMD_display_native_hdr
    VkColorSpaceDisplayNativeAmd = 1000213000,
}

impl From<TextureFormatColorSpace> for vk::ColorSpaceKHR {
    fn from(f: TextureFormatColorSpace) -> Self {
        vk::ColorSpaceKHR::from_raw(f as i32)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ImageTransitionLayout {
    Undefined,
    General,
    ColorAttachment,
    DepthStencilAttachment,
    DepthStencilReadOnly,
    ShaderReadOnly,
    TransferSrc,
    TransferDst,
    Preinitialized,
    Present,
}

impl From<ImageTransitionLayout> for vk::ImageLayout {
    fn from(t: ImageTransitionLayout) -> Self {
        match t {
            ImageTransitionLayout::Undefined => vk::ImageLayout::UNDEFINED,
            ImageTransitionLayout::General => vk::ImageLayout::GENERAL,
            ImageTransitionLayout::ColorAttachment => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ImageTransitionLayout::DepthStencilAttachment => {
                vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
            }
            ImageTransitionLayout::DepthStencilReadOnly => {
                vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL
            }
            ImageTransitionLayout::ShaderReadOnly => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            ImageTransitionLayout::TransferSrc => vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            ImageTransitionLayout::TransferDst => vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            ImageTransitionLayout::Preinitialized => vk::ImageLayout::PREINITIALIZED,
            ImageTransitionLayout::Present => vk::ImageLayout::PRESENT_SRC_KHR,
        }
    }
}
