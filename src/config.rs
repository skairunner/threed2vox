use anyhow::{anyhow, Result};
use clap::ArgMatches;
use std::io::Read;
use std::collections::HashMap;
use std::path::Path;


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
    /// Just the filename portion of the input path
    pub filename: String,
    /// The block to use for occupied voxels. Defaults to stone
    pub block: String,
    /// Rotations in radians
    pub x_rot: f32,
    pub y_rot: f32,
    pub z_rot: f32,
}

impl Config {
    pub fn from_argmatch(args: ArgMatches) -> Result<Self> {
        let input_path = args.value_of("input")
            .ok_or_else(|| anyhow!("No input specified"))?
            .to_string();
        let filename = Path::new(&input_path)
            .file_stem()
            .expect(format!("The path '{}' doesn't seem to contain a file name", input_path).as_str())
            .to_str()
            .unwrap()
            .to_string();

        let version = args.value_of("minecraft version")
            .ok_or_else(|| anyhow!("No version specified"))?;
        let block = args.value_of("block")
            .unwrap_or("stone")
            .to_string();
        let data_version = match version.parse() {
            Ok(n) => n,
            Err(_) => Self::parse_version_string(version)
        };

        // Checks for scale.
        let voxel_size = match args.value_of("scale") {
            Some(s) => {
                let n = s.parse()
                    .unwrap_or(1.0);
                VoxelOption::VoxelSize(n)
            },
            None => {
                // If scale doesn't exist, check for size. If size doesn't exist, default is 1.0
                let s = args.value_of("max_size").unwrap_or("1.0");
                let n = s.parse()
                    .unwrap_or(1.0);
                VoxelOption::MeshSize(n)
            }
        };

        // Rotations
        let x_rot = std::f32::consts::FRAC_PI_2 * (args.occurrences_of("x_rot") as f32);
        let y_rot = std::f32::consts::FRAC_PI_2 * (args.occurrences_of("y_rot") as f32);
        let z_rot = std::f32::consts::FRAC_PI_2 * (args.occurrences_of("z_rot") as f32);
        println!("[INFO] {} {} {}", x_rot, y_rot, z_rot);

        Ok(Self {
            voxel_size,
            data_version,
            input_path,
            block,
            filename,
            x_rot,
            y_rot,
            z_rot
        })
    }

    pub fn parse_version_string(version: &str) -> i32 {
        use strsim::normalized_levenshtein;

        let mut toml_content = String::new();
        let mut file = std::fs::File::open("minecraft_versions.toml")
            .expect("Could not find minecraft_versions.toml");
        file.read_to_string(&mut toml_content).unwrap();
        let index: HashMap<String, i32> = toml::from_str(&toml_content).unwrap();

        // First, try to look it up directly, and if it's in there return it
        if index.contains_key(version) {
            return index[version]
        }

        // Next, try to find the version str with the closest distance.
        let result = index.into_iter()
            .map(|(k, v)| (normalized_levenshtein(&k, version), k, v))
            // Find the max.
            .fold((-1.0, String::new(), 0), |prev, this| {
                if this.0 > prev.0 {
                    this
                } else {
                    prev
                }
            });
        println!("[INFO] Could not find version '{}', using closest match '{}'.", version, result.1);
        result.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Config::parse_version_string("20w20a"), 2536)
    }

    #[test]
    fn prefers_full_match() {
        assert_eq!(Config::parse_version_string("1.16"), 2566)
    }

    #[test]
    fn does_partial_match() {
        assert_eq!(Config::parse_version_string("1.13-"), 1519)
    }
}