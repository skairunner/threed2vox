use std::collections::HashMap;

/// Sparse voxel grid
pub struct VoxelGrid {
    pub dimensions: (i32, i32, i32),
    map: HashMap<(i32, i32, i32), bool>,
}

impl VoxelGrid {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            dimensions: (x, y, z),
            map: Default::default()
        }
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> &bool {
        self.map.get(&(x, y, z))
            .unwrap_or(&false)
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, is_set: bool) {
        self.map.insert((x, y, z), is_set);
    }
}