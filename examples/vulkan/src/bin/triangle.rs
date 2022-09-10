use vulkanite_vulkan::vn;
use vn::*;

use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use tracing::info;
use vulkanite_vulkan::raw::vk::PipelineLayout;

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

    let instance = Instance::new(InstanceCreateInfo {
        application_name: Some("Testing".to_string()),
        engine_name: Some("Acute".to_string()),
        vulkan_version: Version::V1_3,
        render: true,
        ..InstanceCreateInfo::default()
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
        .request_device(DeviceCreateInfo {
            queue_families: vec![
                QueueCreateInfo::new(graphics_family, vec![1.0]),
                QueueCreateInfo::new(transfer_family, vec![0.5]),
            ],
            ..DeviceCreateInfo::default()
        })
        .unwrap();

    let format = surface.formats(&adapter).unwrap().next().unwrap();
    let mut queue = queues.next().unwrap();

    surface
        .configure(
            &device,
            &SurfaceConfig {
                usage: TextureUsages::COLOR_ATTACHMENT,
                format,
                width: window.inner_size().width,
                height: window.inner_size().height,
                mode: PresentMode::Mailbox,
            },
        )
        .unwrap();

    let present_semaphore = device.create_binary_semaphore();
    let render_semaphore = device.create_binary_semaphore();
    let render_fence = device.create_fence();

    let shader = device.create_shader_module(ShaderSource::Wgsl(include_str!("../shader/triangle.wgsl").into()), ShaderCompileInfo::default()).unwrap();

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutInfo {
        flags: PipelineLayoutFlags::empty(),
        bind_group_layouts: &[],
        push_constant_ranges: &[]
    }).unwrap();

    let pipeline = device.create_raster_pipeline(&RasterPipelineInfo {
        layout: &pipeline_layout,
        vertex: ShaderStage {
            module: &shader,
            entry_point: "vs_main"
        },
        vertex_buffers: None,
        fragment: Some(ShaderStage {
            module: &shader,
            entry_point: "fs_main"
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::CounterClock,
            cull_mode: Some(CullModeFlags::BACK),
            polygon_mode: PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false
        },
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        },
        targets: &[]
    }).unwrap();
    event_loop.run(move |event, event_loop, control_flow| match event {
        Event::WindowEvent { event, window_id } => match event {
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

            let mut encoder = device.command_encoder(CommandEncoderInfo { queue: &queue });

            encoder.begin_encoding();

            encoder.frame_transition(
                ImageTransitionLayout::Undefined,
                ImageTransitionLayout::ColorAttachment,
                &frame,
            );
            encoder.begin_rendering(RenderInfo {
                color_attachments: &[RenderAttachmentInfo {
                    load_op: LoadOp::Clear,
                    store_op: StoreOp::Store,
                    clear: ClearOp::Color(Color::RED),
                }],
                frame: &frame,
                offset: (0, 0),
                area: (window.inner_size().width, window.inner_size().height),
            });

            encoder.end_rendering();

            encoder.frame_transition(
                ImageTransitionLayout::ColorAttachment,
                ImageTransitionLayout::Present,
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
