pub mod maps;
pub mod vehicles;

use crate::utils::gpu::{Vertex, Location, include_vectors};
use maps::map::Map;
use vehicles::car::Car;
use wgpu::util::DeviceExt;

pub struct Game {
    pub viewport: [f32; 4],
    pub current_map: Map,
    pub current_map_location: Location,
    pub current_map_vertex_buffer: wgpu::Buffer,
    pub current_map_index_buffer: wgpu::Buffer,
    pub current_map_location_buffer: wgpu::Buffer,
    pub cars: Vec<Car>,
    pub cars_locations: Vec<Vec<Location>>,
    pub cars_vertex_buffers: Vec<wgpu::Buffer>,
    pub cars_index_buffers: Vec<wgpu::Buffer>,
    pub cars_location_buffers: Vec<wgpu::Buffer>,
}

impl Game {
    pub fn create(device: &wgpu::Device) -> Self {
        // let map_vertices: Box<[Vertex]> = vec![ //TODO: load everything from an include_str! like function (include_vertices! and include_indices!)
        //     Vertex {
        //         position: [-5., -5.],
        //         color: [0.1, 0.1, 0.1, 1.],
        //     },
        //     Vertex {
        //         position: [10., -5.],
        //         color: [0.1, 0.1, 0.1, 1.],
        //     },
        //     Vertex {
        //         position: [-5., 10.],
        //         color: [0.1, 0.1, 0.1, 1.],
        //     },
        //     Vertex {
        //         position: [10., 10.],
        //         color: [0.1, 0.1, 0.1, 1.],
        //     },
        //     Vertex {
        //         position: [0., 0.],
        //         color: [0.4, 0.4, 0.4, 1.],
        //     },
        //     Vertex {
        //         position: [0., 10.],
        //         color: [0.4, 0.4, 0.4, 1.],
        //     },
        //     Vertex {
        //         position: [10., 0.],
        //         color: [0.4, 0.4, 0.4, 1.],
        //     },
        //     Vertex {
        //         position: [10., 10.],
        //         color: [0.4, 0.4, 0.4, 1.],
        //     },
        // ].into_boxed_slice();
        
        // let map_indices: Box<[u32]> = vec![0, 1, 2, 3, 3, 4, 4, 5, 6, 7].into_boxed_slice();
    
        let (map_vertices, map_indices) = include_vectors!("vectors/map_0.json");
        
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
        
        let map_location = Location::new();
        
        let map_location_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Location Buffer"),
            contents: bytemuck::cast_slice(&[map_location]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        
        let car_body_vertices: Box<[Vertex]> = vec![
            Vertex {
                position: [0., 0.],
                color: [0.1, 0., 0., 1.],
            },
            Vertex {
                position: [1., 0.],
                color: [0.1, 0., 0., 1.],
            },
            Vertex {
                position: [0., 2.],
                color: [1., 0., 0., 1.],
            },
            Vertex {
                position: [1., 2.],
                color: [1., 0., 0., 1.],
            },
        ].into_boxed_slice();
        
        let car_body_indices: Box<[u32]> = vec![0, 1, 2, 3].into_boxed_slice();
        
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
        
        let new_cars_locations: Vec<Vec<Location>> = vec![vec![Location::new()]];
        
        let new_cars_location_buffer = vec![device.create_buffer_init(&wgpu::util::BufferInitDescriptor { // TODO: Remember to EXPAND for more cars!!!! Use for loop to dynamically make larger pls :)
            label: Some("First Car Location Buffer"),
            contents: bytemuck::cast_slice(&new_cars_locations[0]),
            usage: wgpu::BufferUsages::VERTEX,
        })];
        
        
        Self {
            viewport: [0., 0., 0., 0.],
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
    
    pub fn resize_viewport(&mut self, window_size: &[f32; 2]) {
        self.viewport[0] = window_size[0];
        self.viewport[1] = window_size[1];
        
        if window_size[1] > window_size[0] {
            self.viewport[0] = window_size[1];
        } else {
            self.viewport[1] = window_size[0];
        }
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
    
    pub fn render_objects(&mut self, renderpass: &mut wgpu::RenderPass) {
        renderpass.set_vertex_buffer(0, self.current_map_vertex_buffer.slice(..));
        renderpass.set_vertex_buffer(1, self.current_map_location_buffer.slice(..));
        renderpass.set_index_buffer(self.current_map_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        renderpass.draw_indexed(0..self.current_map.indices.len() as u32, 0, 0..1);

        for i in 0..self.cars.len() { // For each type of car, render it a bunch with instancing
            renderpass.set_vertex_buffer(0, self.cars_vertex_buffers[i].slice(..));
            renderpass.set_vertex_buffer(1, self.cars_location_buffers[i].slice(..));
            renderpass.set_index_buffer(self.cars_index_buffers[i].slice(..), wgpu::IndexFormat::Uint32);
            renderpass.draw_indexed(0..self.cars[i].indices.len() as u32, 0, 0..self.cars_locations[0].len() as _); // TODO: change draw indexed based on Locations of each cars
        }
    }
}