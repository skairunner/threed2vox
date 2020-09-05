use anyhow::{anyhow, Result};
use clap::ArgMatches;


pub enum VoxelOption {
    /// Explicitly define a voxel size
    VoxelSize(f32),
    /// Define the voxel length of the longest axis of the model's AABB.
    MeshSize(f32)
}

/// Pass around configuration options easily.
pub struct Config {
    /// Determines the final size of the schematic
    pub voxel_size: VoxelOption,
    /// Can be derived from a plaintext version, e.g. "Java 1.9.2", or can be supplied directly.
    /// Used to output a schematic
    pub data_version: i32,
    /// The input file. Currently only supports .obj
    pub input_path: String,
}

impl Config {
    pub fn from_argmatch(args: ArgMatches) -> Result<Self> {
        let input_path = args.value_of("input")
            .ok_or_else(|| anyhow!("No input specified"))?
            .to_string();

        Ok(Self {
            voxel_size: VoxelOption::MeshSize(5.0),
            data_version: 0,
            input_path
        })
    }
}