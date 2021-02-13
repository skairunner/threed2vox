use threed2vox::config::{Config, VoxelOption};
use threed2vox::to_schematic;

fn main() {
    let config = Config {
        voxel_size: VoxelOption::MeshSize(20.0),
        data_version: 2566,
        input_path: "models/teapot.obj".to_string(),
        filename: "teapot".to_string(),
        block: "stone".to_string(),
        x_rot: 0.0,
        y_rot: 0.0,
        z_rot: 0.0,
        threads: 4
    };
    let blob = to_schematic(config).unwrap();
    println!("{}", blob.len_bytes());
}