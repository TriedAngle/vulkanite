use std::{io, mem};
use vulkanite_vulkan::vn;

use nalgebra_glm as na;
use tracing::info;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    normals: [f32; 3],
    color: [f32; 3],
}

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
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct MeshPushConstants {
    data: [f32; 4],
    matrix: [[f32; 4]; 4],
}

fn main() {
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();

    info!("Init Tracing");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Triangle Test")
        .with_inner_size(PhysicalSize::new(1080, 720))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

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

    let mut vertex_spv = io::Cursor::new(&include_bytes!("../shader/vert.spv")[..]);
    let mut fragment_spv = io::Cursor::new(&include_bytes!("../shader/frag.spv")[..]);

    let shader_vertex = device
        .create_shader_module(
            vn::ShaderSource::SpirV(&mut vertex_spv),
            vn::ShaderCompileInfo::default(),
        )
        .unwrap();

    let shader_fragment = device
        .create_shader_module(
            vn::ShaderSource::SpirV(&mut fragment_spv),
            vn::ShaderCompileInfo::default(),
        )
        .unwrap();

    let vertex_buffer = device
        .create_buffer_init(&vn::BufferInitInfo {
            label: None,
            contents: bytemuck::cast_slice(VERTICES),
            usage: vn::BufferUsages::VERTEX | vn::BufferUsages::MAP_WRITE,
            sharing: vn::BufferSharing::Exclusive,
        })
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
                front_face: vn::FrontFace::CounterClock,
                cull_mode: None,
                polygon_mode: vn::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
                line_width: 1.0,
            },
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
                color_attachments: &[vn::RenderAttachmentInfo {
                    load_op: vn::LoadOp::Clear,
                    store_op: vn::StoreOp::Store,
                    clear: vn::ClearOp::Color(vn::Color::norm(0.1, 0.2, 0.3, 1.0)),
                }],
                frame: &frame,
                offset: (0, 0),
                area: (surface_config.width, surface_config.height),
            });

            encoder.bind_raster_pipeline(&pipeline);
            encoder.bind_vertex_buffer(0, &vertex_buffer);

            let cam_pos = na::vec3(0.0, 0.0, -2.0);
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
                data: [0.0, 0.0, 0.0, 0.0],
                matrix: matrix.data.0,
            };

            encoder.push_constants(
                &pipeline_layout,
                vn::ShaderStages::VERTEX,
                0,
                bytemuck::cast_slice(&[push_constant]),
            );
            encoder.draw(0..VERTICES.len() as u32, 0..1);

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
