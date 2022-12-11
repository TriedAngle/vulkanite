use std::io;
use vulkanite_vulkan::vn;

use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use tracing::info;

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

    surface
        .configure(
            &device,
            &vn::SurfaceConfig {
                usage: vn::TextureUsages::COLOR_ATTACHMENT,
                format,
                width: window.inner_size().width,
                height: window.inner_size().height,
                mode: vn::PresentMode::Mailbox,
            },
        )
        .unwrap();

    let present_semaphore = device.create_binary_semaphore();
    let render_semaphore = device.create_binary_semaphore();
    let render_fence = device.create_fence();

    // let shader = device
    //     .create_shader_module(
    //         ShaderSource::Wgsl(include_str!("../shader/triangle.wgsl").into()),
    //         ShaderCompileInfo::default(),
    //     )
    //     .unwrap();

    let mut vertex_spv = io::Cursor::new(&include_bytes!("../shader/vert.spv")[..]);
    let mut fragment_spv = io::Cursor::new(&include_bytes!("../shader/frag.spv")[..]);
    let shader_vertex = device
        .create_shader_module(
            vn::ShaderSource::SpirV(&mut vertex_spv),
            vn::ShaderCompileInfo::default()
        ).unwrap();

    let shader_fragment = device
        .create_shader_module(
            vn::ShaderSource::SpirV(&mut fragment_spv),
            vn::ShaderCompileInfo::default()
        ).unwrap();

    let pipeline_layout = device
        .create_pipeline_layout(&vn::PipelineLayoutInfo {
            flags: vn::PipelineLayoutFlags::empty(),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        })
        .unwrap();

    let pipeline = device
        .create_raster_pipeline(&vn::RasterPipelineInfo {
            layout: &pipeline_layout,
            vertex: vn::ShaderStage {
                module: &shader_vertex,
                entry_point: "main",
            },
            vertex_buffers: None,
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
                    clear: vn::ClearOp::Color(vn::Color::GREEN),
                }],
                frame: &frame,
                offset: (0, 0),
                area: (window.inner_size().width, window.inner_size().height),
            });

            encoder.set_raster_pipeline(&pipeline);
            encoder.draw(0..3, 0..1);

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
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
