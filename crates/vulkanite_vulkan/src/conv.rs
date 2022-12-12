use crate::pipeline::{CullModeFlags, FrontFace, PolygonMode, PrimitiveTopology, ShaderStages};
use ash::vk;
use vulkanite_types::BufferSharing;
use vulkanite_types::{BlendComponent, BlendFactor, BlendOperation, VertexFormat};
use vulkanite_types::{BufferUsages, PresentMode, TextureUsages};

pub fn map_shader_stage(stage: ShaderStages) -> vk::ShaderStageFlags {
    let mut flags = vk::ShaderStageFlags::empty();
    if stage.contains(ShaderStages::VERTEX) {
        flags |= vk::ShaderStageFlags::VERTEX;
    }
    if stage.contains(ShaderStages::FRAGMENT) {
        flags |= vk::ShaderStageFlags::FRAGMENT;
    }
    if stage.contains(ShaderStages::COMPUTE) {
        flags |= vk::ShaderStageFlags::COMPUTE;
    }
    flags
}

pub fn map_topology(topology: PrimitiveTopology) -> vk::PrimitiveTopology {
    match topology {
        PrimitiveTopology::PointList => vk::PrimitiveTopology::POINT_LIST,
        PrimitiveTopology::LineList => vk::PrimitiveTopology::LINE_LIST,
        PrimitiveTopology::LineStrip => vk::PrimitiveTopology::LINE_STRIP,
        PrimitiveTopology::TriangleList => vk::PrimitiveTopology::TRIANGLE_LIST,
        PrimitiveTopology::TriangleStrip => vk::PrimitiveTopology::TRIANGLE_STRIP,
        PrimitiveTopology::TriangleFan => vk::PrimitiveTopology::TRIANGLE_FAN,
        PrimitiveTopology::LineListWithAdjacency => vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY,
        PrimitiveTopology::LineStripWithAdjacency => {
            vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY
        }
        PrimitiveTopology::TriangleListWithAdjacency => {
            vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY
        }
        PrimitiveTopology::TriangleStripWithAdjacency => {
            vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY
        }
        PrimitiveTopology::PatchList => vk::PrimitiveTopology::PATCH_LIST,
    }
}

pub fn map_polygon_mode(mode: PolygonMode) -> vk::PolygonMode {
    match mode {
        PolygonMode::Fill => vk::PolygonMode::FILL,
        PolygonMode::Line => vk::PolygonMode::LINE,
        PolygonMode::Point => vk::PolygonMode::POINT,
        PolygonMode::FillRectangleNV => vk::PolygonMode::FILL_RECTANGLE_NV,
    }
}

pub fn map_front_face(front_face: FrontFace) -> vk::FrontFace {
    match front_face {
        FrontFace::Clock => vk::FrontFace::CLOCKWISE,
        FrontFace::CounterClock => vk::FrontFace::COUNTER_CLOCKWISE,
    }
}

pub fn map_cull_face(face: CullModeFlags) -> vk::CullModeFlags {
    match face {
        CullModeFlags::NONE => vk::CullModeFlags::NONE,
        CullModeFlags::FRONT => vk::CullModeFlags::FRONT,
        CullModeFlags::BACK => vk::CullModeFlags::BACK,
        CullModeFlags::FRONT_BACK => vk::CullModeFlags::FRONT_AND_BACK,
        _ => unreachable!(),
    }
}

fn map_blend_factor(factor: BlendFactor) -> vk::BlendFactor {
    match factor {
        BlendFactor::Zero => vk::BlendFactor::ZERO,
        BlendFactor::One => vk::BlendFactor::ONE,
        BlendFactor::Src => vk::BlendFactor::SRC_COLOR,
        BlendFactor::OneMinusSrc => vk::BlendFactor::ONE_MINUS_SRC_COLOR,
        BlendFactor::SrcAlpha => vk::BlendFactor::SRC_ALPHA,
        BlendFactor::OneMinusSrcAlpha => vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        BlendFactor::Dst => vk::BlendFactor::DST_COLOR,
        BlendFactor::OneMinusDst => vk::BlendFactor::ONE_MINUS_DST_COLOR,
        BlendFactor::DstAlpha => vk::BlendFactor::DST_ALPHA,
        BlendFactor::OneMinusDstAlpha => vk::BlendFactor::ONE_MINUS_DST_ALPHA,
        BlendFactor::SrcAlphaSaturated => vk::BlendFactor::SRC_ALPHA_SATURATE,
        BlendFactor::Constant => vk::BlendFactor::CONSTANT_COLOR,
        BlendFactor::OneMinusConstant => vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
    }
}

fn map_blend_op(operation: BlendOperation) -> vk::BlendOp {
    match operation {
        BlendOperation::Add => vk::BlendOp::ADD,
        BlendOperation::Subtract => vk::BlendOp::SUBTRACT,
        BlendOperation::ReverseSubtract => vk::BlendOp::REVERSE_SUBTRACT,
        BlendOperation::Min => vk::BlendOp::MIN,
        BlendOperation::Max => vk::BlendOp::MAX,
    }
}

pub fn map_blend_component(
    component: &BlendComponent,
) -> (vk::BlendOp, vk::BlendFactor, vk::BlendFactor) {
    let op = map_blend_op(component.operation);
    let src = map_blend_factor(component.src_factor);
    let dst = map_blend_factor(component.dst_factor);
    (op, src, dst)
}

pub fn map_vertex_format(vertex_format: VertexFormat) -> vk::Format {
    match vertex_format {
        VertexFormat::Uint8x2 => vk::Format::R8G8_UINT,
        VertexFormat::Uint8x4 => vk::Format::R8G8B8A8_UINT,
        VertexFormat::Sint8x2 => vk::Format::R8G8_SINT,
        VertexFormat::Sint8x4 => vk::Format::R8G8B8A8_SINT,
        VertexFormat::Unorm8x2 => vk::Format::R8G8_UNORM,
        VertexFormat::Unorm8x4 => vk::Format::R8G8B8A8_UNORM,
        VertexFormat::Snorm8x2 => vk::Format::R8G8_SNORM,
        VertexFormat::Snorm8x4 => vk::Format::R8G8B8A8_SNORM,
        VertexFormat::Uint16x2 => vk::Format::R16G16_UINT,
        VertexFormat::Uint16x4 => vk::Format::R16G16B16A16_UINT,
        VertexFormat::Sint16x2 => vk::Format::R16G16_SINT,
        VertexFormat::Sint16x4 => vk::Format::R16G16B16A16_SINT,
        VertexFormat::Unorm16x2 => vk::Format::R16G16_UNORM,
        VertexFormat::Unorm16x4 => vk::Format::R16G16B16A16_UNORM,
        VertexFormat::Snorm16x2 => vk::Format::R16G16_SNORM,
        VertexFormat::Snorm16x4 => vk::Format::R16G16B16A16_SNORM,
        VertexFormat::Float16x2 => vk::Format::R16G16_SFLOAT,
        VertexFormat::Float16x4 => vk::Format::R16G16B16A16_SFLOAT,
        VertexFormat::Float32 => vk::Format::R32_SFLOAT,
        VertexFormat::Float32x2 => vk::Format::R32G32_SFLOAT,
        VertexFormat::Float32x3 => vk::Format::R32G32B32_SFLOAT,
        VertexFormat::Float32x4 => vk::Format::R32G32B32A32_SFLOAT,
        VertexFormat::Uint32 => vk::Format::R32_UINT,
        VertexFormat::Uint32x2 => vk::Format::R32G32_UINT,
        VertexFormat::Uint32x3 => vk::Format::R32G32B32_UINT,
        VertexFormat::Uint32x4 => vk::Format::R32G32B32A32_UINT,
        VertexFormat::Sint32 => vk::Format::R32_SINT,
        VertexFormat::Sint32x2 => vk::Format::R32G32_SINT,
        VertexFormat::Sint32x3 => vk::Format::R32G32B32_SINT,
        VertexFormat::Sint32x4 => vk::Format::R32G32B32A32_SINT,
        VertexFormat::Float64 => vk::Format::R64_SFLOAT,
        VertexFormat::Float64x2 => vk::Format::R64G64_SFLOAT,
        VertexFormat::Float64x3 => vk::Format::R64G64B64_SFLOAT,
        VertexFormat::Float64x4 => vk::Format::R64G64B64A64_SFLOAT,
    }
}

pub fn map_buffer_usage(usage: BufferUsages) -> vk::BufferUsageFlags {
    let mut flags = vk::BufferUsageFlags::empty();

    if usage.contains(BufferUsages::COPY_SRC) {
        flags |= vk::BufferUsageFlags::TRANSFER_SRC;
    }
    if usage.contains(BufferUsages::COPY_DST) {
        flags |= vk::BufferUsageFlags::TRANSFER_DST;
    }
    if usage.contains(BufferUsages::UNIFORM) {
        flags |= vk::BufferUsageFlags::UNIFORM_BUFFER;
    }
    if usage.contains(BufferUsages::STORAGE_READ | BufferUsages::STORAGE_READ_WRITE) {
        flags |= vk::BufferUsageFlags::STORAGE_BUFFER;
    }
    if usage.contains(BufferUsages::INDEX) {
        flags |= vk::BufferUsageFlags::INDEX_BUFFER;
    }
    if usage.contains(BufferUsages::VERTEX) {
        flags |= vk::BufferUsageFlags::VERTEX_BUFFER;
    }
    if usage.contains(BufferUsages::INDIRECT) {
        flags |= vk::BufferUsageFlags::INDIRECT_BUFFER;
    }
    flags
}

pub fn map_buffer_sharing(sharing: BufferSharing) -> vk::SharingMode {
    match sharing {
        BufferSharing::Exclusive => vk::SharingMode::EXCLUSIVE,
        BufferSharing::Concurrent => vk::SharingMode::CONCURRENT,
    }
}

pub fn map_texture_usages(usages: TextureUsages) -> vk::ImageUsageFlags {
    vk::ImageUsageFlags::from_raw(vk::Flags::from(usages.bits()))
}

pub fn map_present_mode(present_mode: PresentMode) -> vk::PresentModeKHR {
    vk::PresentModeKHR::from_raw(present_mode as i32)
}
