use crate::pipeline::vt;
use crate::vn::{DepthAttachment, RenderAttachment};
use ash::vk;

pub fn map_extent2d(extent: vt::Extent2D) -> vk::Extent2D {
    vk::Extent2D {
        width: extent.width,
        height: extent.height,
    }
}

pub fn map_extent3d(extent: vt::Extent3D) -> vk::Extent3D {
    vk::Extent3D {
        width: extent.width,
        height: extent.height,
        depth: extent.depth,
    }
}

pub fn map_texture_dimension(dim: vt::TextureDimension) -> vk::ImageType {
    match dim {
        vt::TextureDimension::D1 => vk::ImageType::TYPE_1D,
        vt::TextureDimension::D2 => vk::ImageType::TYPE_2D,
        vt::TextureDimension::D3 => vk::ImageType::TYPE_3D,
    }
}

pub fn map_texture_subresource_range(
    range: vt::ImageSubresourceRange,
) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::builder()
        .aspect_mask(map_format_aspects(range.aspects))
        .base_mip_level(range.base_mip_level)
        .base_array_layer(range.base_array_layer)
        .layer_count(range.array_layer_count)
        .level_count(range.mip_level_count)
        .build()
}

pub fn map_format_aspects(aspects: vt::TextureAspects) -> vk::ImageAspectFlags {
    let mut flags = vk::ImageAspectFlags::empty();

    if aspects.contains(vt::TextureAspects::COLOR) {
        flags |= vk::ImageAspectFlags::COLOR;
    }

    if aspects.contains(vt::TextureAspects::DEPTH) {
        flags |= vk::ImageAspectFlags::DEPTH;
    }

    if aspects.contains(vt::TextureAspects::STENCIL) {
        flags |= vk::ImageAspectFlags::STENCIL;
    }
    if aspects.contains(vt::TextureAspects::METADATA) {
        flags |= vk::ImageAspectFlags::METADATA;
    }

    flags
}

pub fn map_texture_view_dimension(dim: vt::TextureViewDimension) -> vk::ImageViewType {
    match dim {
        vt::TextureViewDimension::D1 => vk::ImageViewType::TYPE_1D,
        vt::TextureViewDimension::D2 => vk::ImageViewType::TYPE_2D,
        vt::TextureViewDimension::D3 => vk::ImageViewType::TYPE_3D,
        vt::TextureViewDimension::Cube => vk::ImageViewType::CUBE,
        vt::TextureViewDimension::Array1D => vk::ImageViewType::TYPE_1D_ARRAY,
        vt::TextureViewDimension::Array2D => vk::ImageViewType::TYPE_2D_ARRAY,
        vt::TextureViewDimension::ArrayCube => vk::ImageViewType::CUBE_ARRAY,
    }
}

pub fn map_texture_format(format: vt::TextureFormat) -> vk::Format {
    vk::Format::from_raw(format as i32)
}

pub fn map_store_op(op: vt::StoreOp) -> vk::AttachmentStoreOp {
    match op {
        vt::StoreOp::Store => vk::AttachmentStoreOp::STORE,
        vt::StoreOp::DontCare => vk::AttachmentStoreOp::DONT_CARE,
    }
}

pub fn map_render_attachment_info(info: &RenderAttachment) -> vk::RenderingAttachmentInfo {
    let mut attachment = vk::RenderingAttachmentInfo::builder()
        .image_view(info.view.handle)
        .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
        .store_op(map_store_op(info.ops.store));

    match info.ops.load {
        vt::LoadOp::Clear(val) => {
            attachment = attachment
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .clear_value(val.into());
        }
        vt::LoadOp::Load => {
            attachment = attachment.load_op(vk::AttachmentLoadOp::LOAD);
        }
        vt::LoadOp::DontCare => {
            attachment = attachment.load_op(vk::AttachmentLoadOp::DONT_CARE);
        }
    }

    attachment.build()
}

pub fn map_depth_attachment_info(info: &DepthAttachment) -> vk::RenderingAttachmentInfo {
    let mut attachment = vk::RenderingAttachmentInfo::builder()
        .image_view(info.view.handle)
        .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
        .store_op(map_store_op(info.ops.store));

    match info.ops.load {
        vt::LoadOp::Clear(val) => {
            attachment =
                attachment
                    .load_op(vk::AttachmentLoadOp::CLEAR)
                    .clear_value(vk::ClearValue {
                        depth_stencil: vk::ClearDepthStencilValue {
                            depth: val,
                            stencil: 0,
                        },
                    })
        }
        vt::LoadOp::Load => {
            attachment = attachment.load_op(vk::AttachmentLoadOp::LOAD);
        }
        vt::LoadOp::DontCare => {
            attachment = attachment.load_op(vk::AttachmentLoadOp::DONT_CARE);
        }
    }

    attachment.build()
}

pub fn map_shader_stage(stage: vt::ShaderStages) -> vk::ShaderStageFlags {
    let mut flags = vk::ShaderStageFlags::empty();
    if stage.contains(vt::ShaderStages::VERTEX) {
        flags |= vk::ShaderStageFlags::VERTEX;
    }
    if stage.contains(vt::ShaderStages::FRAGMENT) {
        flags |= vk::ShaderStageFlags::FRAGMENT;
    }
    if stage.contains(vt::ShaderStages::COMPUTE) {
        flags |= vk::ShaderStageFlags::COMPUTE;
    }
    flags
}

pub fn map_topology(topology: vt::PrimitiveTopology) -> vk::PrimitiveTopology {
    match topology {
        vt::PrimitiveTopology::PointList => vk::PrimitiveTopology::POINT_LIST,
        vt::PrimitiveTopology::LineList => vk::PrimitiveTopology::LINE_LIST,
        vt::PrimitiveTopology::LineStrip => vk::PrimitiveTopology::LINE_STRIP,
        vt::PrimitiveTopology::TriangleList => vk::PrimitiveTopology::TRIANGLE_LIST,
        vt::PrimitiveTopology::TriangleStrip => vk::PrimitiveTopology::TRIANGLE_STRIP,
        vt::PrimitiveTopology::TriangleFan => vk::PrimitiveTopology::TRIANGLE_FAN,
        vt::PrimitiveTopology::LineListWithAdjacency => {
            vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY
        }
        vt::PrimitiveTopology::LineStripWithAdjacency => {
            vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY
        }
        vt::PrimitiveTopology::TriangleListWithAdjacency => {
            vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY
        }
        vt::PrimitiveTopology::TriangleStripWithAdjacency => {
            vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY
        }
        vt::PrimitiveTopology::PatchList => vk::PrimitiveTopology::PATCH_LIST,
    }
}

pub fn map_polygon_mode(mode: vt::PolygonMode) -> vk::PolygonMode {
    match mode {
        vt::PolygonMode::Fill => vk::PolygonMode::FILL,
        vt::PolygonMode::Line => vk::PolygonMode::LINE,
        vt::PolygonMode::Point => vk::PolygonMode::POINT,
        vt::PolygonMode::FillRectangleNV => vk::PolygonMode::FILL_RECTANGLE_NV,
    }
}

pub fn map_front_face(front_face: vt::FrontFace) -> vk::FrontFace {
    match front_face {
        vt::FrontFace::Clock => vk::FrontFace::CLOCKWISE,
        vt::FrontFace::CounterClock => vk::FrontFace::COUNTER_CLOCKWISE,
    }
}

pub fn map_cull_face(face: vt::CullModeFlags) -> vk::CullModeFlags {
    match face {
        vt::CullModeFlags::NONE => vk::CullModeFlags::NONE,
        vt::CullModeFlags::FRONT => vk::CullModeFlags::FRONT,
        vt::CullModeFlags::BACK => vk::CullModeFlags::BACK,
        vt::CullModeFlags::FRONT_BACK => vk::CullModeFlags::FRONT_AND_BACK,
        _ => unreachable!(),
    }
}

fn map_blend_factor(factor: vt::BlendFactor) -> vk::BlendFactor {
    match factor {
        vt::BlendFactor::Zero => vk::BlendFactor::ZERO,
        vt::BlendFactor::One => vk::BlendFactor::ONE,
        vt::BlendFactor::Src => vk::BlendFactor::SRC_COLOR,
        vt::BlendFactor::OneMinusSrc => vk::BlendFactor::ONE_MINUS_SRC_COLOR,
        vt::BlendFactor::SrcAlpha => vk::BlendFactor::SRC_ALPHA,
        vt::BlendFactor::OneMinusSrcAlpha => vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        vt::BlendFactor::Dst => vk::BlendFactor::DST_COLOR,
        vt::BlendFactor::OneMinusDst => vk::BlendFactor::ONE_MINUS_DST_COLOR,
        vt::BlendFactor::DstAlpha => vk::BlendFactor::DST_ALPHA,
        vt::BlendFactor::OneMinusDstAlpha => vk::BlendFactor::ONE_MINUS_DST_ALPHA,
        vt::BlendFactor::SrcAlphaSaturated => vk::BlendFactor::SRC_ALPHA_SATURATE,
        vt::BlendFactor::Constant => vk::BlendFactor::CONSTANT_COLOR,
        vt::BlendFactor::OneMinusConstant => vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
    }
}

fn map_blend_op(operation: vt::BlendOperation) -> vk::BlendOp {
    match operation {
        vt::BlendOperation::Add => vk::BlendOp::ADD,
        vt::BlendOperation::Subtract => vk::BlendOp::SUBTRACT,
        vt::BlendOperation::ReverseSubtract => vk::BlendOp::REVERSE_SUBTRACT,
        vt::BlendOperation::Min => vk::BlendOp::MIN,
        vt::BlendOperation::Max => vk::BlendOp::MAX,
    }
}

pub fn map_blend_component(
    component: &vt::BlendComponent,
) -> (vk::BlendOp, vk::BlendFactor, vk::BlendFactor) {
    let op = map_blend_op(component.operation);
    let src = map_blend_factor(component.src_factor);
    let dst = map_blend_factor(component.dst_factor);
    (op, src, dst)
}

pub fn map_vertex_format(vertex_format: vt::VertexFormat) -> vk::Format {
    match vertex_format {
        vt::VertexFormat::Uint8x2 => vk::Format::R8G8_UINT,
        vt::VertexFormat::Uint8x4 => vk::Format::R8G8B8A8_UINT,
        vt::VertexFormat::Sint8x2 => vk::Format::R8G8_SINT,
        vt::VertexFormat::Sint8x4 => vk::Format::R8G8B8A8_SINT,
        vt::VertexFormat::Unorm8x2 => vk::Format::R8G8_UNORM,
        vt::VertexFormat::Unorm8x4 => vk::Format::R8G8B8A8_UNORM,
        vt::VertexFormat::Snorm8x2 => vk::Format::R8G8_SNORM,
        vt::VertexFormat::Snorm8x4 => vk::Format::R8G8B8A8_SNORM,
        vt::VertexFormat::Uint16x2 => vk::Format::R16G16_UINT,
        vt::VertexFormat::Uint16x4 => vk::Format::R16G16B16A16_UINT,
        vt::VertexFormat::Sint16x2 => vk::Format::R16G16_SINT,
        vt::VertexFormat::Sint16x4 => vk::Format::R16G16B16A16_SINT,
        vt::VertexFormat::Unorm16x2 => vk::Format::R16G16_UNORM,
        vt::VertexFormat::Unorm16x4 => vk::Format::R16G16B16A16_UNORM,
        vt::VertexFormat::Snorm16x2 => vk::Format::R16G16_SNORM,
        vt::VertexFormat::Snorm16x4 => vk::Format::R16G16B16A16_SNORM,
        vt::VertexFormat::Float16x2 => vk::Format::R16G16_SFLOAT,
        vt::VertexFormat::Float16x4 => vk::Format::R16G16B16A16_SFLOAT,
        vt::VertexFormat::Float32 => vk::Format::R32_SFLOAT,
        vt::VertexFormat::Float32x2 => vk::Format::R32G32_SFLOAT,
        vt::VertexFormat::Float32x3 => vk::Format::R32G32B32_SFLOAT,
        vt::VertexFormat::Float32x4 => vk::Format::R32G32B32A32_SFLOAT,
        vt::VertexFormat::Uint32 => vk::Format::R32_UINT,
        vt::VertexFormat::Uint32x2 => vk::Format::R32G32_UINT,
        vt::VertexFormat::Uint32x3 => vk::Format::R32G32B32_UINT,
        vt::VertexFormat::Uint32x4 => vk::Format::R32G32B32A32_UINT,
        vt::VertexFormat::Sint32 => vk::Format::R32_SINT,
        vt::VertexFormat::Sint32x2 => vk::Format::R32G32_SINT,
        vt::VertexFormat::Sint32x3 => vk::Format::R32G32B32_SINT,
        vt::VertexFormat::Sint32x4 => vk::Format::R32G32B32A32_SINT,
        vt::VertexFormat::Float64 => vk::Format::R64_SFLOAT,
        vt::VertexFormat::Float64x2 => vk::Format::R64G64_SFLOAT,
        vt::VertexFormat::Float64x3 => vk::Format::R64G64B64_SFLOAT,
        vt::VertexFormat::Float64x4 => vk::Format::R64G64B64A64_SFLOAT,
    }
}

pub fn map_buffer_usage(usage: vt::BufferUsages) -> vk::BufferUsageFlags {
    let mut flags = vk::BufferUsageFlags::empty();

    if usage.contains(vt::BufferUsages::COPY_SRC) {
        flags |= vk::BufferUsageFlags::TRANSFER_SRC;
    }
    if usage.contains(vt::BufferUsages::COPY_DST) {
        flags |= vk::BufferUsageFlags::TRANSFER_DST;
    }
    if usage.contains(vt::BufferUsages::UNIFORM) {
        flags |= vk::BufferUsageFlags::UNIFORM_BUFFER;
    }
    if usage.contains(vt::BufferUsages::STORAGE_READ | vt::BufferUsages::STORAGE_READ_WRITE) {
        flags |= vk::BufferUsageFlags::STORAGE_BUFFER;
    }
    if usage.contains(vt::BufferUsages::INDEX) {
        flags |= vk::BufferUsageFlags::INDEX_BUFFER;
    }
    if usage.contains(vt::BufferUsages::VERTEX) {
        flags |= vk::BufferUsageFlags::VERTEX_BUFFER;
    }
    if usage.contains(vt::BufferUsages::INDIRECT) {
        flags |= vk::BufferUsageFlags::INDIRECT_BUFFER;
    }
    flags
}

pub fn map_sharing_mode(sharing: vt::SharingMode) -> vk::SharingMode {
    match sharing {
        vt::SharingMode::Exclusive => vk::SharingMode::EXCLUSIVE,
        vt::SharingMode::Concurrent => vk::SharingMode::CONCURRENT,
    }
}

pub fn map_texture_usages(usages: vt::TextureUsages) -> vk::ImageUsageFlags {
    vk::ImageUsageFlags::from_raw(vk::Flags::from(usages.bits()))
}

pub fn map_present_mode(present_mode: vt::PresentMode) -> vk::PresentModeKHR {
    vk::PresentModeKHR::from_raw(present_mode as i32)
}

pub fn map_depth_function(depth_fn: vt::DepthCompareOperator) -> vk::CompareOp {
    match depth_fn {
        vt::DepthCompareOperator::Never => vk::CompareOp::NEVER,
        vt::DepthCompareOperator::Less => vk::CompareOp::LESS,
        vt::DepthCompareOperator::LessEqual => vk::CompareOp::LESS_OR_EQUAL,
        vt::DepthCompareOperator::Equal => vk::CompareOp::EQUAL,
        vt::DepthCompareOperator::GreaterEqual => vk::CompareOp::GREATER_OR_EQUAL,
        vt::DepthCompareOperator::Greater => vk::CompareOp::GREATER,
        vt::DepthCompareOperator::NotEqual => vk::CompareOp::NOT_EQUAL,
        vt::DepthCompareOperator::Always => vk::CompareOp::ALWAYS,
    }
}
