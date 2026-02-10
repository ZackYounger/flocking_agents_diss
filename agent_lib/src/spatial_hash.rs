use crate::vec::Vec2;
use std::collections::HashMap;

pub struct SpatialHash {
    cell_size: f32,
    cells: HashMap<(i32, i32, i32), Vec<usize>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn build(&mut self, positions: &[Vec2]) {
        self.cells.clear();
        for (i, pos) in positions.iter().enumerate() {
            let key = self.get_key(*pos);
            self.cells.entry(key).or_insert_with(Vec::new).push(i);
        }
    }

    pub fn get_key(&self, pos: Vec2) -> (i32, i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
            (pos.z / self.cell_size).floor() as i32,
        )
    }

    pub fn query(&self, pos: Vec2, radius: f32) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let key = self.get_key(pos);
        
        // Number of cells to check in each direction based on radius
        let cells_to_check = (radius / self.cell_size).ceil() as i32;

        for dx in -cells_to_check..=cells_to_check {
            for dy in -cells_to_check..=cells_to_check {
                for dz in -cells_to_check..=cells_to_check {
                    let nx = key.0 + dx;
                    let ny = key.1 + dy;
                    let nz = key.2 + dz;
                    
                    if let Some(indices) = self.cells.get(&(nx, ny, nz)) {
                        neighbors.extend(indices);
                    }
                }
            }
        }
        neighbors
    }
}

pub trait AccelerationStructureTrait: Send + Sync {
    fn build(&mut self, positions: &[Vec2]);
    fn query(&self, pos: Vec2, radius: f32) -> Vec<usize>;
}

impl AccelerationStructureTrait for SpatialHash {
    fn build(&mut self, positions: &[Vec2]) {
        SpatialHash::build(self, positions);
    }
    fn query(&self, pos: Vec2, radius: f32) -> Vec<usize> {
        SpatialHash::query(self, pos, radius)
    }
}
