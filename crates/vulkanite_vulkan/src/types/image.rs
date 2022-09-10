use ash::vk;
use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    pub struct TextureUsages: u32 {
        const TRANSFER_SRC = 1 << 0;
        const TRANSFER_DST = 1 << 1;
        const SAMPLED = 1 << 2;
        const STORAGE = 1 << 3;
        const COLOR_ATTACHMENT = 1 << 4;
        const DEPTH_STENCIL_ATTACHMENT = 1 << 5;
        const TRANSIENT_ATTACHMENT = 1 << 6;
        const INPUT_ATTACHMENT = 1 << 7;
    }
}

impl From<TextureUsages> for vk::ImageUsageFlags {
    fn from(usages: TextureUsages) -> Self {
        vk::ImageUsageFlags::from_raw(vk::Flags::from(usages.bits))
    }
}

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TextureFormat {
    Undefined = 0,
    R4g4UnormPack8 = 1,
    R4g4b4a4UnormPack16 = 2,
    B4g4r4a4UnormPack16 = 3,
    R5g6b5UnormPack16 = 4,
    B5g6r5UnormPack16 = 5,
    R5g5b5a1UnormPack16 = 6,
    B5g5r5a1UnormPack16 = 7,
    A1r5g5b5UnormPack16 = 8,
    R8Unorm = 9,
    R8Snorm = 10,
    R8Uscaled = 11,
    R8Sscaled = 12,
    R8Uint = 13,
    R8Sint = 14,
    R8Srgb = 15,
    R8g8Unorm = 16,
    R8g8Snorm = 17,
    R8g8Uscaled = 18,
    R8g8Sscaled = 19,
    R8g8Uint = 20,
    R8g8Sint = 21,
    R8g8Srgb = 22,
    R8g8b8Unorm = 23,
    R8g8b8Snorm = 24,
    R8g8b8Uscaled = 25,
    R8g8b8Sscaled = 26,
    R8g8b8Uint = 27,
    R8g8b8Sint = 28,
    R8g8b8Srgb = 29,
    B8g8r8Unorm = 30,
    B8g8r8Snorm = 31,
    B8g8r8Uscaled = 32,
    B8g8r8Sscaled = 33,
    B8g8r8Uint = 34,
    B8g8r8Sint = 35,
    B8g8r8Srgb = 36,
    R8g8b8a8Unorm = 37,
    R8g8b8a8Snorm = 38,
    R8g8b8a8Uscaled = 39,
    R8g8b8a8Sscaled = 40,
    R8g8b8a8Uint = 41,
    R8g8b8a8Sint = 42,
    R8g8b8a8Srgb = 43,
    B8g8r8a8Unorm = 44,
    B8g8r8a8Snorm = 45,
    B8g8r8a8Uscaled = 46,
    B8g8r8a8Sscaled = 47,
    B8g8r8a8Uint = 48,
    B8g8r8a8Sint = 49,
    B8g8r8a8Srgb = 50,
    A8b8g8r8UnormPack32 = 51,
    A8b8g8r8SnormPack32 = 52,
    A8b8g8r8UscaledPack32 = 53,
    A8b8g8r8SscaledPack32 = 54,
    A8b8g8r8UintPack32 = 55,
    A8b8g8r8SintPack32 = 56,
    A8b8g8r8SrgbPack32 = 57,
    A2r10g10b10UnormPack32 = 58,
    A2r10g10b10SnormPack32 = 59,
    A2r10g10b10UscaledPack32 = 60,
    A2r10g10b10SscaledPack32 = 61,
    A2r10g10b10UintPack32 = 62,
    A2r10g10b10SintPack32 = 63,
    A2b10g10r10UnormPack32 = 64,
    A2b10g10r10SnormPack32 = 65,
    A2b10g10r10UscaledPack32 = 66,
    A2b10g10r10SscaledPack32 = 67,
    A2b10g10r10UintPack32 = 68,
    A2b10g10r10SintPack32 = 69,
    R16Unorm = 70,
    R16Snorm = 71,
    R16Uscaled = 72,
    R16Sscaled = 73,
    R16Uint = 74,
    R16Sint = 75,
    R16Sfloat = 76,
    R16g16Unorm = 77,
    R16g16Snorm = 78,
    R16g16Uscaled = 79,
    R16g16Sscaled = 80,
    R16g16Uint = 81,
    R16g16Sint = 82,
    R16g16Sfloat = 83,
    R16g16b16Unorm = 84,
    R16g16b16Snorm = 85,
    R16g16b16Uscaled = 86,
    R16g16b16Sscaled = 87,
    R16g16b16Uint = 88,
    R16g16b16Sint = 89,
    R16g16b16Sfloat = 90,
    R16g16b16a16Unorm = 91,
    R16g16b16a16Snorm = 92,
    R16g16b16a16Uscaled = 93,
    R16g16b16a16Sscaled = 94,
    R16g16b16a16Uint = 95,
    R16g16b16a16Sint = 96,
    R16g16b16a16Sfloat = 97,
    R32Uint = 98,
    R32Sint = 99,
    R32Sfloat = 100,
    R32g32Uint = 101,
    R32g32Sint = 102,
    R32g32Sfloat = 103,
    R32g32b32Uint = 104,
    R32g32b32Sint = 105,
    R32g32b32Sfloat = 106,
    R32g32b32a32Uint = 107,
    R32g32b32a32Sint = 108,
    R32g32b32a32Sfloat = 109,
    R64Uint = 110,
    R64Sint = 111,
    R64Sfloat = 112,
    R64g64Uint = 113,
    R64g64Sint = 114,
    R64g64Sfloat = 115,
    R64g64b64Uint = 116,
    R64g64b64Sint = 117,
    R64g64b64Sfloat = 118,
    R64g64b64a64Uint = 119,
    R64g64b64a64Sint = 120,
    R64g64b64a64Sfloat = 121,
    B10g11r11UfloatPack32 = 122,
    E5b9g9r9UfloatPack32 = 123,
    D16Unorm = 124,
    X8D24UnormPack32 = 125,
    D32Sfloat = 126,
    S8Uint = 127,
    D16UnormS8Uint = 128,
    D24UnormS8Uint = 129,
    D32SfloatS8Uint = 130,
    Bc1RgbUnormBlock = 131,
    Bc1RgbSrgbBlock = 132,
    Bc1RgbaUnormBlock = 133,
    Bc1RgbaSrgbBlock = 134,
    Bc2UnormBlock = 135,
    Bc2SrgbBlock = 136,
    Bc3UnormBlock = 137,
    Bc3SrgbBlock = 138,
    Bc4UnormBlock = 139,
    Bc4SnormBlock = 140,
    Bc5UnormBlock = 141,
    Bc5SnormBlock = 142,
    Bc6hUfloatBlock = 143,
    Bc6hSfloatBlock = 144,
    Bc7UnormBlock = 145,
    Bc7SrgbBlock = 146,
    Etc2R8g8b8UnormBlock = 147,
    Etc2R8g8b8SrgbBlock = 148,
    Etc2R8g8b8a1UnormBlock = 149,
    Etc2R8g8b8a1SrgbBlock = 150,
    Etc2R8g8b8a8UnormBlock = 151,
    Etc2R8g8b8a8SrgbBlock = 152,
    EacR11UnormBlock = 153,
    EacR11SnormBlock = 154,
    EacR11g11UnormBlock = 155,
    EacR11g11SnormBlock = 156,
    Astc4x4UnormBlock = 157,
    Astc4x4SrgbBlock = 158,
    Astc5x4UnormBlock = 159,
    Astc5x4SrgbBlock = 160,
    Astc5x5UnormBlock = 161,
    Astc5x5SrgbBlock = 162,
    Astc6x5UnormBlock = 163,
    Astc6x5SrgbBlock = 164,
    Astc6x6UnormBlock = 165,
    Astc6x6SrgbBlock = 166,
    Astc8x5UnormBlock = 167,
    Astc8x5SrgbBlock = 168,
    Astc8x6UnormBlock = 169,
    Astc8x6SrgbBlock = 170,
    Astc8x8UnormBlock = 171,
    Astc8x8SrgbBlock = 172,
    Astc10x5UnormBlock = 173,
    Astc10x5SrgbBlock = 174,
    Astc10x6UnormBlock = 175,
    Astc10x6SrgbBlock = 176,
    Astc10x8UnormBlock = 177,
    Astc10x8SrgbBlock = 178,
    Astc10x10UnormBlock = 179,
    Astc10x10SrgbBlock = 180,
    Astc12x10UnormBlock = 181,
    Astc12x10SrgbBlock = 182,
    Astc12x12UnormBlock = 183,
    Astc12x12SrgbBlock = 184,
}

impl From<TextureFormat> for vk::Format {
    fn from(format: TextureFormat) -> Self {
        vk::Format::from_raw(format as i32)
    }
}

impl From<i32> for TextureFormat {
    fn from(val: i32) -> Self {
        match val {
            1 => TextureFormat::R4g4UnormPack8,
            2 => TextureFormat::R4g4b4a4UnormPack16,
            3 => TextureFormat::B4g4r4a4UnormPack16,
            4 => TextureFormat::R5g6b5UnormPack16,
            5 => TextureFormat::B5g6r5UnormPack16,
            6 => TextureFormat::R5g5b5a1UnormPack16,
            7 => TextureFormat::B5g5r5a1UnormPack16,
            8 => TextureFormat::A1r5g5b5UnormPack16,
            9 => TextureFormat::R8Unorm,
            10 => TextureFormat::R8Snorm,
            11 => TextureFormat::R8Uscaled,
            12 => TextureFormat::R8Sscaled,
            13 => TextureFormat::R8Uint,
            14 => TextureFormat::R8Sint,
            15 => TextureFormat::R8Srgb,
            16 => TextureFormat::R8g8Unorm,
            17 => TextureFormat::R8g8Snorm,
            18 => TextureFormat::R8g8Uscaled,
            19 => TextureFormat::R8g8Sscaled,
            20 => TextureFormat::R8g8Uint,
            21 => TextureFormat::R8g8Sint,
            22 => TextureFormat::R8g8Srgb,
            23 => TextureFormat::R8g8b8Unorm,
            24 => TextureFormat::R8g8b8Snorm,
            25 => TextureFormat::R8g8b8Uscaled,
            26 => TextureFormat::R8g8b8Sscaled,
            27 => TextureFormat::R8g8b8Uint,
            28 => TextureFormat::R8g8b8Sint,
            29 => TextureFormat::R8g8b8Srgb,
            30 => TextureFormat::B8g8r8Unorm,
            31 => TextureFormat::B8g8r8Snorm,
            32 => TextureFormat::B8g8r8Uscaled,
            33 => TextureFormat::B8g8r8Sscaled,
            34 => TextureFormat::B8g8r8Uint,
            35 => TextureFormat::B8g8r8Sint,
            36 => TextureFormat::B8g8r8Srgb,
            37 => TextureFormat::R8g8b8a8Unorm,
            38 => TextureFormat::R8g8b8a8Snorm,
            39 => TextureFormat::R8g8b8a8Uscaled,
            40 => TextureFormat::R8g8b8a8Sscaled,
            41 => TextureFormat::R8g8b8a8Uint,
            42 => TextureFormat::R8g8b8a8Sint,
            43 => TextureFormat::R8g8b8a8Srgb,
            44 => TextureFormat::B8g8r8a8Unorm,
            45 => TextureFormat::B8g8r8a8Snorm,
            46 => TextureFormat::B8g8r8a8Uscaled,
            47 => TextureFormat::B8g8r8a8Sscaled,
            48 => TextureFormat::B8g8r8a8Uint,
            49 => TextureFormat::B8g8r8a8Sint,
            50 => TextureFormat::B8g8r8a8Srgb,
            51 => TextureFormat::A8b8g8r8UnormPack32,
            52 => TextureFormat::A8b8g8r8SnormPack32,
            53 => TextureFormat::A8b8g8r8UscaledPack32,
            54 => TextureFormat::A8b8g8r8SscaledPack32,
            55 => TextureFormat::A8b8g8r8UintPack32,
            56 => TextureFormat::A8b8g8r8SintPack32,
            57 => TextureFormat::A8b8g8r8SrgbPack32,
            58 => TextureFormat::A2r10g10b10UnormPack32,
            59 => TextureFormat::A2r10g10b10SnormPack32,
            60 => TextureFormat::A2r10g10b10UscaledPack32,
            61 => TextureFormat::A2r10g10b10SscaledPack32,
            62 => TextureFormat::A2r10g10b10UintPack32,
            63 => TextureFormat::A2r10g10b10SintPack32,
            64 => TextureFormat::A2b10g10r10UnormPack32,
            65 => TextureFormat::A2b10g10r10SnormPack32,
            66 => TextureFormat::A2b10g10r10UscaledPack32,
            67 => TextureFormat::A2b10g10r10SscaledPack32,
            68 => TextureFormat::A2b10g10r10UintPack32,
            69 => TextureFormat::A2b10g10r10SintPack32,
            70 => TextureFormat::R16Unorm,
            71 => TextureFormat::R16Snorm,
            72 => TextureFormat::R16Uscaled,
            73 => TextureFormat::R16Sscaled,
            74 => TextureFormat::R16Uint,
            75 => TextureFormat::R16Sint,
            76 => TextureFormat::R16Sfloat,
            77 => TextureFormat::R16g16Unorm,
            78 => TextureFormat::R16g16Snorm,
            79 => TextureFormat::R16g16Uscaled,
            80 => TextureFormat::R16g16Sscaled,
            81 => TextureFormat::R16g16Uint,
            82 => TextureFormat::R16g16Sint,
            83 => TextureFormat::R16g16Sfloat,
            84 => TextureFormat::R16g16b16Unorm,
            85 => TextureFormat::R16g16b16Snorm,
            86 => TextureFormat::R16g16b16Uscaled,
            87 => TextureFormat::R16g16b16Sscaled,
            88 => TextureFormat::R16g16b16Uint,
            89 => TextureFormat::R16g16b16Sint,
            90 => TextureFormat::R16g16b16Sfloat,
            91 => TextureFormat::R16g16b16a16Unorm,
            92 => TextureFormat::R16g16b16a16Snorm,
            93 => TextureFormat::R16g16b16a16Uscaled,
            94 => TextureFormat::R16g16b16a16Sscaled,
            95 => TextureFormat::R16g16b16a16Uint,
            96 => TextureFormat::R16g16b16a16Sint,
            97 => TextureFormat::R16g16b16a16Sfloat,
            98 => TextureFormat::R32Uint,
            99 => TextureFormat::R32Sint,
            100 => TextureFormat::R32Sfloat,
            101 => TextureFormat::R32g32Uint,
            102 => TextureFormat::R32g32Sint,
            103 => TextureFormat::R32g32Sfloat,
            104 => TextureFormat::R32g32b32Uint,
            105 => TextureFormat::R32g32b32Sint,
            106 => TextureFormat::R32g32b32Sfloat,
            107 => TextureFormat::R32g32b32a32Uint,
            108 => TextureFormat::R32g32b32a32Sint,
            109 => TextureFormat::R32g32b32a32Sfloat,
            110 => TextureFormat::R64Uint,
            111 => TextureFormat::R64Sint,
            112 => TextureFormat::R64Sfloat,
            113 => TextureFormat::R64g64Uint,
            114 => TextureFormat::R64g64Sint,
            115 => TextureFormat::R64g64Sfloat,
            116 => TextureFormat::R64g64b64Uint,
            117 => TextureFormat::R64g64b64Sint,
            118 => TextureFormat::R64g64b64Sfloat,
            119 => TextureFormat::R64g64b64a64Uint,
            120 => TextureFormat::R64g64b64a64Sint,
            121 => TextureFormat::R64g64b64a64Sfloat,
            122 => TextureFormat::B10g11r11UfloatPack32,
            123 => TextureFormat::E5b9g9r9UfloatPack32,
            124 => TextureFormat::D16Unorm,
            125 => TextureFormat::X8D24UnormPack32,
            126 => TextureFormat::D32Sfloat,
            127 => TextureFormat::S8Uint,
            128 => TextureFormat::D16UnormS8Uint,
            129 => TextureFormat::D24UnormS8Uint,
            130 => TextureFormat::D32SfloatS8Uint,
            131 => TextureFormat::Bc1RgbUnormBlock,
            132 => TextureFormat::Bc1RgbSrgbBlock,
            133 => TextureFormat::Bc1RgbaUnormBlock,
            134 => TextureFormat::Bc1RgbaSrgbBlock,
            135 => TextureFormat::Bc2UnormBlock,
            136 => TextureFormat::Bc2SrgbBlock,
            137 => TextureFormat::Bc3UnormBlock,
            138 => TextureFormat::Bc3SrgbBlock,
            139 => TextureFormat::Bc4UnormBlock,
            140 => TextureFormat::Bc4SnormBlock,
            141 => TextureFormat::Bc5UnormBlock,
            142 => TextureFormat::Bc5SnormBlock,
            143 => TextureFormat::Bc6hUfloatBlock,
            144 => TextureFormat::Bc6hSfloatBlock,
            145 => TextureFormat::Bc7UnormBlock,
            146 => TextureFormat::Bc7SrgbBlock,
            147 => TextureFormat::Etc2R8g8b8UnormBlock,
            148 => TextureFormat::Etc2R8g8b8SrgbBlock,
            149 => TextureFormat::Etc2R8g8b8a1UnormBlock,
            150 => TextureFormat::Etc2R8g8b8a1SrgbBlock,
            151 => TextureFormat::Etc2R8g8b8a8UnormBlock,
            152 => TextureFormat::Etc2R8g8b8a8SrgbBlock,
            153 => TextureFormat::EacR11UnormBlock,
            154 => TextureFormat::EacR11SnormBlock,
            155 => TextureFormat::EacR11g11UnormBlock,
            156 => TextureFormat::EacR11g11SnormBlock,
            157 => TextureFormat::Astc4x4UnormBlock,
            158 => TextureFormat::Astc4x4SrgbBlock,
            159 => TextureFormat::Astc5x4UnormBlock,
            160 => TextureFormat::Astc5x4SrgbBlock,
            161 => TextureFormat::Astc5x5UnormBlock,
            162 => TextureFormat::Astc5x5SrgbBlock,
            163 => TextureFormat::Astc6x5UnormBlock,
            164 => TextureFormat::Astc6x5SrgbBlock,
            165 => TextureFormat::Astc6x6UnormBlock,
            166 => TextureFormat::Astc6x6SrgbBlock,
            167 => TextureFormat::Astc8x5UnormBlock,
            168 => TextureFormat::Astc8x5SrgbBlock,
            169 => TextureFormat::Astc8x6UnormBlock,
            170 => TextureFormat::Astc8x6SrgbBlock,
            171 => TextureFormat::Astc8x8UnormBlock,
            172 => TextureFormat::Astc8x8SrgbBlock,
            173 => TextureFormat::Astc10x5UnormBlock,
            174 => TextureFormat::Astc10x5SrgbBlock,
            175 => TextureFormat::Astc10x6UnormBlock,
            176 => TextureFormat::Astc10x6SrgbBlock,
            177 => TextureFormat::Astc10x8UnormBlock,
            178 => TextureFormat::Astc10x8SrgbBlock,
            179 => TextureFormat::Astc10x10UnormBlock,
            180 => TextureFormat::Astc10x10SrgbBlock,
            181 => TextureFormat::Astc12x10UnormBlock,
            182 => TextureFormat::Astc12x10SrgbBlock,
            183 => TextureFormat::Astc12x12UnormBlock,
            184 => TextureFormat::Astc12x12SrgbBlock,
            _ => TextureFormat::Undefined,
        }
    }
}

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

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PresentMode {
    Immediate = 0,
    Mailbox = 1,
    Fifo = 2,
    FifoRelaxed = 3,
}

impl From<i32> for PresentMode {
    fn from(val: i32) -> Self {
        match val {
            0 => Self::Immediate,
            1 => Self::Mailbox,
            2 => Self::Fifo,
            3 => Self::FifoRelaxed,
            _ => panic!("Invalid value for Present Mode"),
        }
    }
}

impl From<PresentMode> for vk::PresentModeKHR {
    fn from(p: PresentMode) -> Self {
        vk::PresentModeKHR::from_raw(p as i32)
    }
}

pub struct Texture {
    pub(crate) handle: vk::Image,
    pub(crate) allocation: Option<gpu_allocator::vulkan::Allocation>,
    pub(crate) usage: TextureUsages,
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

#[cfg(test)]
mod image_type_tests {
    use super::*;
    use ash::vk;

    #[test]
    fn t_usages() {
        let usages = TextureUsages::TRANSFER_DST | TextureUsages::COLOR_ATTACHMENT;
        assert!(usages.contains(TextureUsages::TRANSFER_DST));
        assert!(usages.contains(TextureUsages::COLOR_ATTACHMENT));
        assert!(!usages.contains(TextureUsages::TRANSFER_SRC));
        let vk_usages = vk::ImageUsageFlags::from(usages);
        assert!(vk_usages.contains(vk::ImageUsageFlags::TRANSFER_DST));
        assert!(vk_usages.contains(vk::ImageUsageFlags::COLOR_ATTACHMENT));
        assert!(!vk_usages.contains(vk::ImageUsageFlags::TRANSFER_SRC));
    }

    #[test]
    fn t_format() {
        let format = TextureFormat::A2b10g10r10UscaledPack32;
        let vk_format = vk::Format::from(format);
        assert_eq!(vk_format, vk::Format::A2B10G10R10_USCALED_PACK32);
    }
}
