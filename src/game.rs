pub mod maps;
pub mod vehicles;

use crate::utils::gpu::{Vertex, Location, include_vectors};
use maps::map::Map;
use vehicles::car::Car;

pub struct Game {
    viewport: [f32; 4],
    current_map: Map,
    cars: Vec<Car>,
}

impl Game {
    pub fn create() -> Self {
        let (map_vertices, map_indices) = include_vectors!("vectors/map_0.json");
    
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
        
        let new_cars: Vec<Car> = vec![Car::new(car_body_vertices, car_body_indices)];
        
        
        Self {
            viewport: [0., 0., 0., 0.],
            cars: new_cars,
            current_map: Map::new(map_vertices, map_indices),
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
}