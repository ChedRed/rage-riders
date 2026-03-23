use std::sync::Arc;
use std::thread;
use std::time::Duration;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub mod utils;
use utils::gpu::{Vertex, Location};

use crate::game::Game;

pub mod game;

struct State {
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    content: game::Game,
    // location_bind_group: wgpu::BindGroup,
    window: Arc<Window>,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();
        
        let content: game::Game = Game::create(&device);

        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let raster_shader =
            device.create_shader_module(wgpu::include_wgsl!("shaders/main.wgsl").into());

        // let car_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Car Vertex Buffer"),
        //     contents: bytemuck::cast_slice(CAR_BODY_VERTICES),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        // let car_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Car Index Buffer"),
        //     contents: bytemuck::cast_slice(CAR_BODY_INDICES),
        //     usage: wgpu::BufferUsages::INDEX,
        // });

        // let map_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Map Vertex Buffer"),
        //     contents: bytemuck::cast_slice(MAP_VERTICES),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        // let map_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Map Index Buffer"),
        //     contents: bytemuck::cast_slice(MAP_INDICES),
        //     usage: wgpu::BufferUsages::INDEX,
        // });

        // let mut car_location: Vec<Location> = Vec::new();

        // car_location.push(Location::new());
        // car_location[0].position = [10., 0.];

        // let mut map_location: Vec<Location> = Vec::new();

        // map_location.push(Location::new());
        // map_location[0].position = [0., 0.];

        // let car_location_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Car Locations Buffer"),
        //     contents: bytemuck::cast_slice(&car_location),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        // let map_location_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Map Location Buffer"),
        //     contents: bytemuck::cast_slice(&map_location),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        // let location_bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     label: Some("Location Bind Group Layout"),
        //     entries: &[
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStages::VERTEX,
        //             ty: wgpu::BindingType::Buffer {
        //                 ty: wgpu::BufferBindingType::Uniform,
        //                 has_dynamic_offset: false,
        //                 min_binding_size: None,
        //             },
        //             count: None,
        //         }
        //     ],
        // });

        // let location_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     label: Some("Location Bind Group"),
        //     layout: &location_bind_layout,
        //     entries: &[
        //         wgpu::BindGroupEntry {
        //             binding: 0,
        //             resource: location_buffer.as_entire_binding(),
        //         }
        //     ],
        // });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Layout for Primary Render Pipeline"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Primary Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &raster_shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), Location::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &raster_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                // cull_mode: Some(wgpu::Face::Front),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },

            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        // let num_map_indices = MAP_INDICES.len() as u32;
        // let num_car_indices = CAR_BODY_INDICES.len() as u32;

        let state = State {
            surface,
            surface_format,
            device,
            queue,
            size,
            render_pipeline,
            content,
            window,
        };

        state.configure_surface();
        state
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            // present_mode: wgpu::PresentMode::Mailbox,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }

    fn render(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Error: failed to acquire the next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        renderpass.set_pipeline(&self.render_pipeline);
        // renderpass.set_bind_group(0, &self.location_bind_group, &[]);

        self.content.render_objects(&mut renderpass);
        
        drop(renderpass);
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested! Exiting application...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                state.get_window().request_redraw();
                thread::sleep(Duration::from_millis(6));
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
            }
            _ => (),
        }
    }
}

fn main() {
    let events = EventLoop::new().unwrap();
    events.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    match events.run_app(&mut app) {
        Ok(()) => println!("A-OK!"),
        Err(error) => eprintln!("Error: {error:?}"),
    }
}
