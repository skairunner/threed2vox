use std::collections::HashMap;

use nbt::Value;
use serde::Serialize;

use crate::voxel_grid::VoxelGrid;

trait NBTIfy {
    /// Convert the voxel grid into a suitable NBT format
    /// # Arguments
    /// * `grid`: The VoxelGrid to use
    /// * `block`: The Block ID string to fill non-empty cells with
    fn convert(grid: &VoxelGrid, block: &str) -> nbt::Value;
}

/// Schematic version post-1.8, pre-1.13
/// We don't support pre-1.8 because unsure what the schematic format for that is.
/// As defined https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-1.md
struct SchematicV1;

/// Schematic version post-1.13
/// As defined https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-2.md
struct SchematicV2;

impl NBTIfy for SchematicV2 {
    fn convert(grid: &VoxelGrid, block: &str) -> Value {
        let mut root: HashMap<String, Value> = HashMap::new();
        root.insert("Version".to_string(), Value::Int(0));

        nbt::Value::Compound(root)
    }
}