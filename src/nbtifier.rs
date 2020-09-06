use std::collections::HashMap;
use std::time::SystemTime;

use nbt::Value;
use serde::Serialize;

use crate::voxel_grid::VoxelGrid;
use crate::config::Config;

pub trait NBTIfy {
    /// Convert the voxel grid into a suitable NBT format
    /// # Arguments
    /// * `grid`: The VoxelGrid to use
    /// * `block`: The Block ID string to fill non-empty cells with
    fn convert(grid: &VoxelGrid, config: &Config) -> nbt::Value;
}

pub fn varint_from_int(mut i: u32) -> Vec<u8> {
    let mut output = vec!();
    while (i & 128) != 0 {
        output.push((i & 127 | 128) as u8);
        i >>= 7;
    }
    output.push(i as u8);
    output
}

pub fn varint_from_intarray(array: Vec<u32>) -> Vec<u8> {
    let mut output = vec!();
    for i in array {
        output.append(&mut varint_from_int(i));
    }
    output
}

/// Convert a Vec<u8> to Vec<i8> without copying
/// Borrowed from https://stackoverflow.com/questions/59707349/cast-vector-of-i8-to-vector-of-u8-in-rust
pub fn bytearray_from_varint(array: Vec<u8>) -> Vec<i8> {
    let mut v = std::mem::ManuallyDrop::new(array);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}

/// Schematic version post-1.8, pre-1.13
/// We don't support pre-1.8 because unsure what the schematic format for that is.
/// As defined https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-1.md
pub struct SchematicV1;

/// Schematic version post-1.13
/// As defined https://github.com/SpongePowered/Schematic-Specification/blob/master/versions/schematic-2.md
pub struct SchematicV2;

impl NBTIfy for SchematicV2 {
    fn convert(grid: &VoxelGrid, config: &Config) -> Value {
        let mut root: HashMap<String, Value> = HashMap::new();
        root.insert("Version".to_string(), Value::Int(2));
        root.insert("DataVersion".to_string(), Value::Int(config.data_version));

        let mut metadata = HashMap::new();
        metadata.insert("Name".to_string(), Value::String(config.filename.clone()));
        metadata.insert("Author".to_string(), Value::String("threed2vox".to_string()));
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        metadata.insert("Date".to_string(), Value::Long(now as i64));
        metadata.insert("RequiredMods".to_string(), Value::List(Vec::new()));
        root.insert("Metadata".to_string(), Value::Compound(metadata));

        let (x, y, z) = grid.dimensions;
        root.insert("Width".to_string(), Value::Short(x as i16));
        root.insert("Height".to_string(), Value::Short(y as i16));
        root.insert("Length".to_string(), Value::Short(z as i16));

        root.insert("PaletteMax".to_string(), Value::Int(2));
        let mut palette: HashMap<String, Value> = HashMap::new();
        palette.insert("minecraft:air".to_string(), Value::Int(0));
        palette.insert(config.block.clone(), Value::Int(1));
        root.insert("Palette".to_string(), Value::Compound(palette));

        let mut block_data: Vec<u32> = Vec::new();
        for z in 0..grid.dimensions.2 {
            for y in 0..grid.dimensions.1 {
                for x in 0..grid.dimensions.0 {
                    let id = match grid.get(x, y, z) {
                        true => 1,
                        false => 0
                    };
                    block_data.push(id);
                }
            }
        }
        let block_data = varint_from_intarray(block_data);
        root.insert("BlockData".to_string(), Value::ByteArray(bytearray_from_varint(block_data)));

        nbt::Value::Compound(root)
    }
}