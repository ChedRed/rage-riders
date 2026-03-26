pub mod maps;
pub mod vehicles;

use crate::utils::gpu::{Vertex, Location, include_object};
use maps::map::Map;
use vehicles::car::Car;
use wgpu::util::DeviceExt;
use std::cmp::{min, max};
use chrono;


#[repr(C)]
#[derive(Copy, Clone, Debug, serde::Deserialize, bytemuck::Pod, bytemuck::Zeroable)]
pub struct View {
    pub time: [f32; 2],
    pub scale: [f32; 2],
    pub position: [f32; 2],
    pub rotation: [f32; 2],
}

impl View {
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
    view: View,
    window: [f32; 4],
    view_buffer: wgpu::Buffer,
    view_bind_group: wgpu::BindGroup,
    pub view_bind_layout: wgpu::BindGroupLayout,
    
    current_map: Map,
    current_map_location: Location,
    current_map_vertex_buffer: wgpu::Buffer,
    current_map_index_buffer: wgpu::Buffer,
    current_map_location_buffer: wgpu::Buffer,
    cars: Vec<Car>,
    cars_locations: Vec<Vec<Location>>,
    cars_vertex_buffers: Vec<wgpu::Buffer>,
    cars_index_buffers: Vec<wgpu::Buffer>,
    cars_location_buffers: Vec<wgpu::Buffer>,
}

impl Content {
    pub fn create(device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) -> Self {        
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
        
        let mut new_cars_locations: Vec<Vec<Location>> = vec![vec![Location::new()]];
        new_cars_locations[0][0].center = car_body_center;
        
        let new_cars_location_buffer = vec![device.create_buffer_init(&wgpu::util::BufferInitDescriptor { // TODO: Remember to EXPAND for more cars!!!! Use for loop to dynamically make larger pls :)
            label: Some("First Car Location Buffer"),
            contents: bytemuck::cast_slice(&new_cars_locations[0]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        })];        
        
        let mut new_view: View = View::new();
        let min: f32 = min(size.width, size.height) as f32;
        let max: f32 = max(size.width, size.height) as f32;
        new_view.scale = [size.height as f32 / max, size.width as f32 / max];
        
        let new_window: [f32; 4] = [(size.width as f32 - min) / 2., (size.height as f32 - min) / 2., min, min];
        
        let new_view_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Viewport Buffer"),
            contents: bytemuck::cast_slice(&[new_view]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        let new_view_bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let new_view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Viewport Bind Group"),
            layout: &new_view_bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: new_view_buffer.as_entire_binding(),
                },
            ],
        });
        
        Self {
            init_time: chrono::Utc::now(),
            last_time: chrono::Utc::now(),
            view: new_view,
            window: new_window,
            view_buffer: new_view_buffer,
            view_bind_group: new_view_bind_group,
            view_bind_layout: new_view_bind_layout,
            current_map: map,
            current_map_location: map_location,
            current_map_vertex_buffer: map_vertex_buffer,
            current_map_index_buffer: map_index_buffer,
            current_map_location_buffer: map_location_buffer,
            cars: new_cars,
            cars_locations: new_cars_locations,
            cars_vertex_buffers: new_cars_vertex_buffer,
            cars_index_buffers: new_cars_index_buffer,
            cars_location_buffers: new_cars_location_buffer,
        }
    }
    
    pub fn resize_viewport(&mut self, window_size: &[u32; 2]) {
        let min: f32 = min(window_size[0], window_size[1]) as f32;
        let max: f32 = max(window_size[0], window_size[1]) as f32;
        
        self.view.scale = [window_size[1] as f32 / max, window_size[0] as f32 / max];
        self.window = [(window_size[0] as f32 - min) / 2., (window_size[1] as f32 - min) / 2., min, min];
        
    }
    
    pub fn load_car(&mut self, new_car: Car) {
        self.cars.push(new_car);
    }
    
    pub fn load_map(&mut self, new_map: Map) {
        self.current_map = new_map;
    }
    
    pub fn move_car(&mut self, index: usize, movement: &[f32; 2]) {
        self.cars[index].position[0] += movement[0];
        self.cars[index].position[1] += movement[1];
    }
    
    pub fn update_objects(&mut self, queue: &mut wgpu::Queue) {
        for i in 0..self.cars.len() {
            queue.write_buffer(&self.cars_location_buffers[i], 0, bytemuck::cast_slice(&self.cars_locations[i]));
        }
    }
    
    pub fn render_objects(&mut self, queue: &mut wgpu::Queue, renderpass: &mut wgpu::RenderPass) {
        renderpass.set_viewport(self.window[0], self.window[1], self.window[2], self.window[3], 0., 1.);
        
        self.view.time[0] = chrono::Utc::now().signed_duration_since(self.init_time).as_seconds_f32();
        self.view.time[1] = chrono::Utc::now().signed_duration_since(self.last_time).as_seconds_f32();
        queue.write_buffer(&self.view_buffer, 0, bytemuck::bytes_of(&[self.view]));
        
        renderpass.set_bind_group(0, &self.view_bind_group, &[]);
        renderpass.set_vertex_buffer(0, self.current_map_vertex_buffer.slice(..));
        renderpass.set_vertex_buffer(1, self.current_map_location_buffer.slice(..));
        renderpass.set_index_buffer(self.current_map_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        renderpass.draw_indexed(0..self.current_map.indices.len() as u32, 0, 0..1);

        for i in 0..self.cars.len() { // For each type of car, render it a bunch with instancing
            renderpass.set_vertex_buffer(0, self.cars_vertex_buffers[i].slice(..));
            renderpass.set_vertex_buffer(1, self.cars_location_buffers[i].slice(..));
            renderpass.set_index_buffer(self.cars_index_buffers[i].slice(..), wgpu::IndexFormat::Uint32);
            renderpass.draw_indexed(0..self.cars[i].indices.len() as u32, 0, 0..self.cars_locations[i].len() as _);
        }
        
        self.last_time = chrono::Utc::now();
    }
}