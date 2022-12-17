use crate::color::Color;
use crate::pipeline::vt;
use crate::vn::TextureView;
use ash::vk;

#[derive(Debug, Copy, Clone)]
pub struct RenderInfo<'a> {
    pub color_attachments: &'a [RenderAttachment<'a>],
    pub depth_attachment: Option<DepthAttachment<'a>>,
    pub stencil_attachment: Option<StencilAttachment>,
    pub offset: (i32, i32),
    pub area: (u32, u32),
}

#[derive(Debug, Copy, Clone)]
pub struct RenderAttachment<'a> {
    pub view: &'a TextureView,
    pub ops: vt::Operations<Color>,
}

#[derive(Debug, Copy, Clone)]
pub struct DepthAttachment<'a> {
    pub view: &'a TextureView,
    pub ops: vt::Operations<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct StencilAttachment {
    pub image: Option<vk::ImageView>,
    pub ops: vt::Operations<f32>,
}
