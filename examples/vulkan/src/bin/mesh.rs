use std::{io, mem};
use vulkanite_vulkan::vn;

use nalgebra_glm as na;
use tracing::info;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}

const DEPTH_FORMAT: vn::TextureFormat = vn::TextureFormat::D32Sfloat;

impl Vertex {
    pub fn desc<'a>() -> vn::VertexBufferLayout<'a> {
        vn::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as vn::BufferAddress,
            step_mode: vn::VertexStepMode::Vertex,
            attributes: &[
                vn::VertexAttribute {
                    format: vn::VertexFormat::Float32x3,
                    offset: 0,
                    location: 0,
                },
                vn::VertexAttribute {
                    format: vn::VertexFormat::Float32x3,
                    offset: mem::size_of::<[f32; 3]>() as vn::BufferAddress,
                    location: 1,
                },
                vn::VertexAttribute {
                    format: vn::VertexFormat::Float32x3,
                    offset: (mem::size_of::<[f32; 3]>() * 2) as vn::BufferAddress,
                    location: 2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct MeshPushConstants {
    matrix: [[f32; 4]; 4],
}

fn main() {
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();

    info!("Init Tracing");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Depth + Model Test")
        .with_inner_size(PhysicalSize::new(1080, 720))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let mut options = tobj::LoadOptions::default();
    options.triangulate = true;

    let (models, materials) = tobj::load_obj("examples/vulkan/src/resource/tree.obj", &options)
        .expect("Failed to OBJ load file");

    let model = models.first().unwrap();
    let mesh = &model.mesh;

    let mut vertices = mesh
        .indices
        .iter()
        .map(|_| Vertex::default())
        .collect::<Vec<_>>();

    for idx in 0..mesh.indices.len() {
        let indices_at_idx = mesh.indices[idx] as usize;
        vertices[idx].position[0] = mesh.positions[indices_at_idx * 3 + 0];
        vertices[idx].position[1] = mesh.positions[indices_at_idx * 3 + 1];
        vertices[idx].position[2] = mesh.positions[indices_at_idx * 3 + 2];

        let indices_at_idx_normal = mesh.normal_indices[idx] as usize;

        vertices[idx].normal[0] = mesh.normals[indices_at_idx_normal * 3 + 0];
        vertices[idx].normal[1] = mesh.normals[indices_at_idx_normal * 3 + 1];
        vertices[idx].normal[2] = mesh.normals[indices_at_idx_normal * 3 + 2];

        vertices[idx].color[0] = mesh.normals[indices_at_idx_normal * 3 + 0];
        vertices[idx].color[1] = mesh.normals[indices_at_idx_normal * 3 + 1];
        vertices[idx].color[2] = mesh.normals[indices_at_idx_normal * 3 + 2];
    }

    let instance = vn::Instance::new(vn::InstanceCreateInfo {
        application_name: Some("Testing".to_string()),
        engine_name: Some("Acute".to_string()),
        vulkan_version: vn::Version::V1_3,
        render: true,
        window: Some(&window),
        ..vn::InstanceCreateInfo::default()
    })
    .unwrap();

    let adapter = instance.adapters().next().unwrap();
    let mut surface = instance.create_surface(&window).unwrap();

    let graphics_family = adapter
        .queue_families()
        .find(|queue| queue.supports_graphics())
        .unwrap();

    let transfer_family = adapter
        .queue_families()
        .find(|queue| queue.supports_transfer() && !queue.supports_graphics())
        .unwrap();

    let (device, mut queues) = adapter
        .request_device(vn::DeviceCreateInfo {
            queue_families: vec![
                vn::QueueCreateInfo::new(graphics_family, vec![1.0]),
                vn::QueueCreateInfo::new(transfer_family, vec![0.5]),
            ],
            ..vn::DeviceCreateInfo::default()
        })
        .unwrap();

    let format = surface.formats(&adapter).unwrap().next().unwrap();
    let mut queue = queues.next().unwrap();

    let mut surface_config = vn::SurfaceConfig {
        usage: vn::TextureUsages::COLOR_ATTACHMENT,
        format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        mode: vn::PresentMode::Mailbox,
    };

    surface.configure(&device, &surface_config).unwrap();

    let shader_vertex = device
        .create_shader_module(vn::ShaderSource::Glsl {
            content: include_str!("../shader/mesh.vert").into(),
            kind: vn::ShaderKind::Vertex,
            entry: "main",
        })
        .unwrap();

    let shader_fragment = device
        .create_shader_module(vn::ShaderSource::Glsl {
            content: include_str!("../shader/mesh.frag").into(),
            kind: vn::ShaderKind::Fragment,
            entry: "main",
        })
        .unwrap();

    let vertex_buffer = device
        .create_buffer_init(&vn::BufferInitInfo {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: vn::BufferUsages::VERTEX | vn::BufferUsages::MAP_WRITE,
            sharing: vn::SharingMode::Exclusive,
        })
        .unwrap();

    let mut depth_texture = device
        .create_texture(&vn::TextureInfo {
            dimension: vn::TextureDimension::D2,
            format: DEPTH_FORMAT,
            size: vn::Extent3D {
                width: surface_config.width,
                height: surface_config.height,
                depth: 1,
            },
            mip_levels: 1,
            samples: 1,
            usage: vn::TextureUsages::DEPTH_STENCIL_ATTACHMENT,
            sharing: vn::SharingMode::Exclusive,
        })
        .unwrap();

    let mut depth_view = device
        .create_texture_view(
            &vn::TextureViewInfo {
                dimension: vn::TextureViewDimension::D2,
                format: DEPTH_FORMAT,
                range: vn::ImageSubresourceRange {
                    aspects: vn::TextureAspects::DEPTH,
                    base_mip_level: 0,
                    mip_level_count: 1,
                    base_array_layer: 0,
                    array_layer_count: 1,
                },
            },
            &depth_texture,
        )
        .unwrap();

    let pipeline_layout = device
        .create_pipeline_layout(&vn::PipelineLayoutInfo {
            flags: vn::PipelineLayoutFlags::empty(),
            bind_group_layouts: &[],
            push_constant_ranges: &[vn::PushConstantRange {
                stages: vn::ShaderStages::VERTEX,
                range: 0..mem::size_of::<MeshPushConstants>() as u32,
            }],
        })
        .unwrap();

    let pipeline = device
        .create_raster_pipeline(&vn::RasterPipelineInfo {
            layout: &pipeline_layout,
            vertex: vn::ShaderStage {
                module: &shader_vertex,
                entry_point: "main",
            },
            vertex_buffers: &[Vertex::desc()],
            fragment: Some(vn::ShaderStage {
                module: &shader_fragment,
                entry_point: "main",
            }),
            primitive: vn::PrimitiveState {
                topology: vn::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: vn::FrontFace::Clock,
                cull_mode: Some(vn::CullModeFlags::BACK),
                polygon_mode: vn::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
                line_width: 1.0,
            },
            depth_stencil: Some(vn::DepthStencilState {
                format: DEPTH_FORMAT,
                write: true,
                depth_compare: vn::DepthCompareOperator::Less,
                bias: vn::DepthBiasState {
                    constant: 0.0,
                    slope: 0.0,
                    clamp: 0.0,
                },
                read_mask: 0,
                write_mask: 0,
            }),
            multisample: vn::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            targets: &[vn::ColorTargetState {
                format,
                blend: Some(vn::BlendState {
                    color: vn::BlendComponent::REPLACE,
                    alpha: vn::BlendComponent::REPLACE,
                }),
                write_mask: vn::ColorWrites::ALL,
            }],
        })
        .unwrap();

    let present_semaphore = device.create_binary_semaphore();
    let render_semaphore = device.create_binary_semaphore();
    let render_fence = device.create_fence();
    let mut frame_count = 0;

    event_loop.run(move |event, event_loop, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { ref input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    if key == VirtualKeyCode::Escape && input.state == ElementState::Pressed {
                        *control_flow = ControlFlow::Exit
                    }
                }
            }
            WindowEvent::Resized(new_size) => {
                if new_size.width > 0 && new_size.height > 0 {
                    surface_config.width = new_size.width;
                    surface_config.height = new_size.height;
                    surface.configure(&device, &surface_config).unwrap();
                }

                depth_texture = device
                    .create_texture(&vn::TextureInfo {
                        dimension: vn::TextureDimension::D2,
                        format: DEPTH_FORMAT,
                        size: vn::Extent3D {
                            width: surface_config.width,
                            height: surface_config.height,
                            depth: 1,
                        },
                        mip_levels: 1,
                        samples: 1,
                        usage: vn::TextureUsages::DEPTH_STENCIL_ATTACHMENT,
                        sharing: vn::SharingMode::Exclusive,
                    })
                    .unwrap();

                depth_view = device
                    .create_texture_view(
                        &vn::TextureViewInfo {
                            dimension: vn::TextureViewDimension::D2,
                            format: DEPTH_FORMAT,
                            range: vn::ImageSubresourceRange {
                                aspects: vn::TextureAspects::DEPTH,
                                base_mip_level: 0,
                                mip_level_count: 1,
                                base_array_layer: 0,
                                array_layer_count: 1,
                            },
                        },
                        &depth_texture,
                    )
                    .unwrap();
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                if new_inner_size.width > 0 && new_inner_size.height > 0 {
                    surface_config.width = new_inner_size.width;
                    surface_config.height = new_inner_size.height;
                    surface.configure(&device, &surface_config).unwrap();
                }
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            render_fence.wait_reset(1000).unwrap();

            let frame = surface
                .acquire_frame(1000, Some(&present_semaphore), None)
                .unwrap()
                .unwrap();

            let mut encoder = device.command_encoder(vn::CommandEncoderInfo { queue: &queue });

            encoder.begin_encoding();

            encoder.frame_transition(
                vn::ImageTransitionLayout::Undefined,
                vn::ImageTransitionLayout::ColorAttachment,
                Some(vn::StageFlags::TOP_OF_PIPE),
                None,
                Some(vn::StageFlags::COLOR_ATTACHMENT_OUTPUT),
                Some(vn::AccessFlags::COLOR_ATTACHMENT_WRITE),
                &frame,
            );

            encoder.begin_rendering(vn::RenderInfo {
                color_attachments: &[vn::RenderAttachment {
                    view: frame.view(),
                    ops: vn::Operations {
                        load: vn::LoadOp::Clear(vn::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: vn::StoreOp::Store,
                    },
                }],
                depth_attachment: Some(vn::DepthAttachment {
                    view: &depth_view,
                    ops: vn::Operations {
                        load: vn::LoadOp::Clear(1.0),
                        store: vn::StoreOp::Store,
                    },
                }),
                stencil_attachment: None,
                offset: (0, 0),
                area: (surface_config.width, surface_config.height),
            });

            encoder.bind_raster_pipeline(&pipeline);
            encoder.bind_vertex_buffer(0, &vertex_buffer);

            let cam_pos = na::vec3(0.0, -6.0, -27.0);
            let view = na::translate(&na::Mat4::identity(), &cam_pos);
            let mut projection = na::perspective(
                70.0 * std::f32::consts::PI / 180.0,
                1700.0 / 900.0,
                0.1,
                200.0,
            );
            let model = na::rotate(
                &na::Mat4::identity(),
                frame_count as f32 * 0.4 * std::f32::consts::PI / 180.0,
                &na::vec3(0.0, 1.0, 0.0),
            );
            let matrix = projection * view * model;

            let push_constant = MeshPushConstants {
                matrix: matrix.data.0,
            };

            encoder.push_constants(
                &pipeline_layout,
                vn::ShaderStages::VERTEX,
                0,
                bytemuck::cast_slice(&[push_constant]),
            );
            encoder.draw(0..vertices.len() as u32, 0..1);

            encoder.end_rendering();

            encoder.frame_transition(
                vn::ImageTransitionLayout::ColorAttachment,
                vn::ImageTransitionLayout::Present,
                Some(vn::StageFlags::COLOR_ATTACHMENT_OUTPUT),
                Some(vn::AccessFlags::COLOR_ATTACHMENT_WRITE),
                Some(vn::StageFlags::BOTTOM_OF_PIPE),
                None,
                &frame,
            );

            queue
                .submit(
                    [encoder.finish()],
                    &[&render_semaphore],
                    &[&present_semaphore],
                    Some(&render_fence),
                )
                .unwrap();

            frame
                .present(&queue, &surface, &[&render_semaphore])
                .unwrap();

            frame_count += 1;
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::LoopDestroyed => {}
        _ => {}
    });
}
