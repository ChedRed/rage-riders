pub mod maps;
pub mod vehicles;
pub mod physics;

use crate::utils::{gpu::{Location, Vertex, include_object}, transform::Vector2};
use maps::map::Map;
use vehicles::car::Car;
use wgpu::util::DeviceExt;
use std::{cmp::{max, min}, collections};
use chrono;

pub struct Control {
    pub binds: Vec<winit::keyboard::KeyCode>,
    pub state: bool,
}

impl Control {
    pub fn new(new_binds: Vec<winit::keyboard::KeyCode>) -> Self {
        Self {
            binds: new_binds,
            state: false,
        }
    }
}

pub struct Displacement {
    pub position: Vector2,
    pub rotation: f32,
}

impl Displacement {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(),
            rotation: 0.,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, serde::Deserialize, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUView {
    pub time: [f32; 2],
    pub scale: [f32; 2],
    pub position: [f32; 2],
    pub rotation: [f32; 2],
}

impl GPUView {
    pub fn new() -> Self {
        Self {
            time: [0., 0.],
            scale: [1., 1.],
            position: [0., 0.],
            rotation: [0., 0.],
        }
    }
}

pub struct Content {
    init_time: chrono::DateTime<chrono::Utc>,
    last_time: chrono::DateTime<chrono::Utc>,
    displacement: Displacement,
    gpu_view: GPUView,
    window: [f32; 4],
    gpu_view_buffer: wgpu::Buffer,
    gpu_view_bind_group: wgpu::BindGroup,
    pub gpu_view_bind_layout: wgpu::BindGroupLayout,
    
    current_map: Map,
    current_map_vertex_buffer: wgpu::Buffer,
    current_map_index_buffer: wgpu::Buffer,
    current_map_location_buffer: wgpu::Buffer,
    cars: Vec<Car>,
    focused_car: [usize; 2],
    cars_displacements: Vec<Vec<Displacement>>,
    cars_gpu_locations: Vec<Vec<Location>>,
    cars_vertex_buffers: Vec<wgpu::Buffer>,
    cars_index_buffers: Vec<wgpu::Buffer>,
    cars_location_buffers: Vec<wgpu::Buffer>,
    
    pub controls: collections::HashMap<String, Control>,
}

impl Content {
    pub fn create(device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) -> Self {
        let mut new_controls: collections::HashMap<String, Control> = collections::HashMap::new();
        new_controls.insert("Forward".to_string(), Control::new(vec![winit::keyboard::KeyCode::KeyW]));
        new_controls.insert("Backward".to_string(), Control::new(vec![winit::keyboard::KeyCode::KeyS]));
        new_controls.insert("Left".to_string(), Control::new(vec![winit::keyboard::KeyCode::KeyA]));
        new_controls.insert("Right".to_string(), Control::new(vec![winit::keyboard::KeyCode::KeyD]));
        
        let (map_vertices, map_indices, map_center) = include_object!("vectors/map_0.vec");
        
        let mut map_location: Location = Location::new();
        map_location.center = map_center;
        
        let map_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Vertex Buffer"),
            contents: bytemuck::cast_slice(&map_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        let map_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Index Buffer"),
            contents: bytemuck::cast_slice(&map_indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        let map: Map = Map::new(map_vertices, map_indices);
        
        let map_location_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Location Buffer"),
            contents: bytemuck::cast_slice(&[map_location]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        
        let (car_body_vertices, car_body_indices, car_body_center) = include_object!("vectors/car_0.vec");
        
        let new_cars_vertex_buffer = vec![device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("First Car Vertex Buffer"),
            contents: bytemuck::cast_slice(&car_body_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })];
        
        let new_cars_index_buffer = vec![device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("First Car Index Buffer"),
            contents: bytemuck::cast_slice(&car_body_indices),
            usage: wgpu::BufferUsages::INDEX,
        })];
        
        let new_cars: Vec<Car> = vec![Car::new(car_body_vertices, car_body_indices)];
        
        let new_cars_displacements: Vec<Vec<Displacement>> = vec![vec![Displacement::new()]];
        let mut new_cars_gpu_locations: Vec<Vec<Location>> = vec![vec![Location::new()]];
        new_cars_gpu_locations[0][0].center = car_body_center;
        
        let new_cars_location_buffer = vec![device.create_buffer_init(&wgpu::util::BufferInitDescriptor { // TODO: Remember to EXPAND for more cars!!!! Use for loop to dynamically make larger pls :)
            label: Some("First Car Location Buffer"),
            contents: bytemuck::cast_slice(&new_cars_gpu_locations[0]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })];        
        
        let new_displacement: Displacement = Displacement::new();
        let mut new_gpu_view: GPUView = GPUView::new();
        let min: f32 = min(size.width, size.height) as f32;
        let max: f32 = max(size.width, size.height) as f32;
        new_gpu_view.scale = [size.height as f32 / max, size.width as f32 / max];
        
        let new_window: [f32; 4] = [(size.width as f32 - min) / 2., (size.height as f32 - min) / 2., min, min];
        
        let new_gpu_view_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Viewport Buffer"),
            contents: bytemuck::cast_slice(&[new_gpu_view]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        let new_gpu_view_bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Viewport Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let new_gpu_view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Viewport Bind Group"),
            layout: &new_gpu_view_bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: new_gpu_view_buffer.as_entire_binding(),
                },
            ],
        });
        
        Self {
            init_time: chrono::Utc::now(),
            last_time: chrono::Utc::now(),
            displacement: new_displacement,
            gpu_view: new_gpu_view,
            window: new_window,
            gpu_view_buffer: new_gpu_view_buffer,
            gpu_view_bind_group: new_gpu_view_bind_group,
            gpu_view_bind_layout: new_gpu_view_bind_layout,
            
            current_map: map,
            current_map_vertex_buffer: map_vertex_buffer,
            current_map_index_buffer: map_index_buffer,
            current_map_location_buffer: map_location_buffer,
            cars: new_cars,
            focused_car: [0, 0],
            cars_displacements: new_cars_displacements,
            cars_gpu_locations: new_cars_gpu_locations,
            cars_vertex_buffers: new_cars_vertex_buffer,
            cars_index_buffers: new_cars_index_buffer,
            cars_location_buffers: new_cars_location_buffer,
            controls: new_controls,
        }
    }
    
    pub fn resize_viewport(&mut self, window_size: &[u32; 2]) {
        let min: f32 = min(window_size[0], window_size[1]) as f32;
        let max: f32 = max(window_size[0], window_size[1]) as f32;
        
        self.gpu_view.scale = [window_size[1] as f32 / max, window_size[0] as f32 / max];
        self.window = [(window_size[0] as f32 - min) / 2., (window_size[1] as f32 - min) / 2., min, min];
        
    }
    
    pub fn load_car(&mut self, new_car: Car) {
        self.cars.push(new_car);
    }
    
    pub fn load_map(&mut self, new_map: Map) {
        self.current_map = new_map;
    }
    
    pub fn move_car(&mut self, variant: usize, index: usize, distance: f32, angle: f32) {
        self.cars_displacements[variant][index].rotation += angle;
        self.cars_displacements[variant][index].position.onward(distance, self.cars_gpu_locations[variant][index].rotation[0]);
        self.cars_gpu_locations[variant][index].rotation[0] = self.cars_displacements[variant][index].rotation;
        self.cars_gpu_locations[variant][index].position[0] = self.cars_displacements[variant][index].position.x;
        self.cars_gpu_locations[variant][index].position[1] = self.cars_displacements[variant][index].position.y;
    }
    
    pub fn update_objects(&mut self, queue: &mut wgpu::Queue) {
        
        if self.controls.get("Forward").unwrap().state {
            self.move_car(self.focused_car[0], self.focused_car[1], 0.1, 0.);
        }
        if self.controls.get("Backward").unwrap().state {
            self.move_car(self.focused_car[0], self.focused_car[1], -0.1, 0.);
        }
        if self.controls.get("Left").unwrap().state {
            self.move_car(self.focused_car[0], self.focused_car[1], 0., 0.1);
        }
        if self.controls.get("Right").unwrap().state {
            self.move_car(self.focused_car[0], self.focused_car[1], 0., -0.1);
        }
        
        
        for i in 0..self.cars.len() {
            queue.write_buffer(&self.cars_location_buffers[i], 0, bytemuck::cast_slice(&self.cars_gpu_locations[i]));
        }
        
        let target_position: Vector2 = self.cars_displacements[self.focused_car[0]][self.focused_car[1]].position;
        self.displacement.position.onward(0.02 * target_position.distance(self.displacement.position), target_position.angle_to(self.displacement.position));
        
        self.gpu_view.position[0] = self.displacement.position.x;
        self.gpu_view.position[1] = self.displacement.position.y;
    }
    
    pub fn render_objects(&mut self, queue: &mut wgpu::Queue, renderpass: &mut wgpu::RenderPass) {
        renderpass.set_viewport(self.window[0], self.window[1], self.window[2], self.window[3], 0., 1.);
        
        self.gpu_view.time[0] = chrono::Utc::now().signed_duration_since(self.init_time).as_seconds_f32();
        self.gpu_view.time[1] = chrono::Utc::now().signed_duration_since(self.last_time).as_seconds_f32();
        queue.write_buffer(&self.gpu_view_buffer, 0, bytemuck::bytes_of(&[self.gpu_view]));
        
        renderpass.set_bind_group(0, &self.gpu_view_bind_group, &[]);
        renderpass.set_vertex_buffer(0, self.current_map_vertex_buffer.slice(..));
        renderpass.set_vertex_buffer(1, self.current_map_location_buffer.slice(..));
        renderpass.set_index_buffer(self.current_map_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        renderpass.draw_indexed(0..self.current_map.indices.len() as u32, 0, 0..1);

        for i in 0..self.cars.len() { // For each type of car, render it a bunch with instancing
            renderpass.set_vertex_buffer(0, self.cars_vertex_buffers[i].slice(..));
            renderpass.set_vertex_buffer(1, self.cars_location_buffers[i].slice(..));
            renderpass.set_index_buffer(self.cars_index_buffers[i].slice(..), wgpu::IndexFormat::Uint32);
            renderpass.draw_indexed(0..self.cars[i].indices.len() as u32, 0, 0..self.cars_gpu_locations[i].len() as _);
        }
        
        self.last_time = chrono::Utc::now();
    }
}