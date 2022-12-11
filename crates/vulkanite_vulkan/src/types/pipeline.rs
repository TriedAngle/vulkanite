// pub struct RenderPipeline {
//     handle: vk::Pipeline,
// }
//
// pub struct ComputePipeline {
//     handle: vk::Pipeline
// }
//
// pub struct RenderInfo {
//     pub render_area: [u32; 2],
//     pub layer_count: u32,
//     pub view_mask: u32,
//     // pub color_attachment
// }
//
// #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
// pub struct ResourceBinding {
//     pub group: u32,
//     pub binding: u32,
// }
//
// #[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
// pub struct BindTarget {
//     pub space: u8,
//     pub register: u32,
//     pub binding_array_size: Option<u32>,
// }
//
// #[derive(Debug)]
// pub struct BindGroupLayout {
//     handle: vk::DescriptorSetLayout,
//     desc_count: gpu_descriptor::DescriptorTotalCount,
//     types: Box<[(vk::DescriptorType, u32)]>,
//     binding_arrays: Vec<(u32, NonZeroU32)>,
//     requires_update_after_bind: bool,
// }
//
// pub type BindingMap = std::collections::BTreeMap<ResourceBinding, BindTarget>;
//
// bitflags!(
//     /// Pipeline layout creation flags.
//     pub struct PipelineLayoutFlags: u32 {
//         /// Include support for base vertex/instance drawing.
//         const BASE_VERTEX_INSTANCE = 1 << 0;
//         /// Include support for num work groups builtin.
//         const NUM_WORK_GROUPS = 1 << 1;
//     }
// );
//
// #[derive(Debug, Clone)]
// pub struct PipelineLayoutDescriptor<'a> {
//     pub flags: PipelineLayoutFlags,
//     pub bind_group_layouts: &'a [&'a BindGroupLayout],
//     // pub push_constant_ranges: &'a []
// }
//
// pub struct PipelineLayout {
//     handle: vk::PipelineLayout,
//     binding_arrays: BindingMap,
// }

use crate::color::Color;
use crate::surface::Frame;
use ash::vk;




#[derive(Copy, Clone)]
pub struct RenderInfo<'a> {
    pub color_attachments: &'a [RenderAttachmentInfo],
    pub frame: &'a Frame,
    pub offset: (i32, i32),
    pub area: (u32, u32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RenderAttachmentInfo {
    pub load_op: LoadOp,
    pub store_op: StoreOp,
    pub clear: ClearOp,
}

impl RenderAttachmentInfo {
    pub(crate) fn to_vulkan(&self, view: vk::ImageView) -> vk::RenderingAttachmentInfo {
        vk::RenderingAttachmentInfo::builder()
            .image_view(view)
            .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL_KHR)
            .load_op(self.load_op.into())
            .store_op(self.store_op.into())
            .clear_value(self.clear.into())
            .build()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClearOp {
    Color(Color),
    DepthStencil((f32, u32)),
}

impl From<ClearOp> for vk::ClearValue {
    fn from(op: ClearOp) -> Self {
        match op {
            ClearOp::Color(color) => color.into(),
            ClearOp::DepthStencil((depth, stencil)) => vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue { depth, stencil },
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LoadOp {
    Load = vk::AttachmentLoadOp::LOAD.as_raw(),
    Clear = vk::AttachmentLoadOp::CLEAR.as_raw(),
    DontCare = vk::AttachmentLoadOp::DONT_CARE.as_raw(),
}

impl From<LoadOp> for vk::AttachmentLoadOp {
    #[inline]
    fn from(val: LoadOp) -> Self {
        Self::from_raw(val as i32)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum StoreOp {
    Store = vk::AttachmentStoreOp::STORE.as_raw(),
    DontCare = vk::AttachmentStoreOp::DONT_CARE.as_raw(),
}

impl From<StoreOp> for ash::vk::AttachmentStoreOp {
    #[inline]
    fn from(val: StoreOp) -> Self {
        Self::from_raw(val as i32)
    }
}
