use simplelog::{ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use threed2vox::config::{Config, VoxelOption};
use threed2vox::to_schematic;

fn main() {
    let config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Error)
        .build();
    TermLogger::init(LevelFilter::Debug, config, TerminalMode::Mixed);

    let config = Config {
        voxel_size: VoxelOption::MeshSize(15.0),
        data_version: 2566,
        input_path: "models/teapot.obj".to_string(),
        filename: "teapot".to_string(),
        block: "stone".to_string(),
        x_rot: 0.0,
        y_rot: 0.0,
        z_rot: 0.0,
        threads: 1,
    };
    let blob = to_schematic(config).unwrap();
    println!("{}", blob.len_bytes());
}
