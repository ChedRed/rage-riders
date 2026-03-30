use std::sync::Arc;
use std::thread;
use std::time::Duration;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::platform::scancode::PhysicalKeyExtScancode;
use winit::window::{Window, WindowId};
use chrono;

pub mod utils;
use utils::gpu::{Vertex, Location};

use crate::content::Content;

pub mod content;

struct State {
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    content: content::Content,
    window: Arc<Window>,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();
        
        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];
        
        let content: content::Content = Content::create(&device, size);

        let raster_shader = device.create_shader_module(wgpu::include_wgsl!("shaders/main.wgsl").into());

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Layout for Primary Render Pipeline"),
            bind_group_layouts: &[&content.view_bind_layout],
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
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
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
        let resize_scale: &[u32; 2] = &[self.size.width, self.size.height];
        self.content.resize_viewport(&resize_scale);
    }
    
    fn keyboard_inputs(&mut self, code: winit::keyboard::KeyCode, state: bool) {
        if code == winit::keyboard::KeyCode::KeyW {
            self.content.controls[0].state = state;
        } else if code == winit::keyboard::KeyCode::KeyS {
            self.content.controls[1].state = state;
        } else if code == winit::keyboard::KeyCode::KeyA {
            self.content.controls[2].state = state;
        } else if code == winit::keyboard::KeyCode::KeyD {
            self.content.controls[3].state = state;
        }
    }

    fn render(&mut self) {
        self.content.update_objects(&mut self.queue);
        
        let surface_texture = self.surface.get_current_texture().expect("Error: failed to acquire the next swapchain texture");
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
        
        self.content.render_objects(&mut self.queue, &mut renderpass);
        
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
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let superstate = self.state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested! Exiting application...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                superstate.render();
                superstate.get_window().request_redraw();
                thread::sleep(Duration::from_millis(6));
            }
            WindowEvent::Resized(size) => {
                superstate.resize(size);
            }
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => superstate.keyboard_inputs(code, key_state.is_pressed()),
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
