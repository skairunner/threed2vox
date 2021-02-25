use simplelog::{ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use threed2vox::config::{Config, VoxelOption};
use threed2vox::nbtifier::SchematicV2;
use threed2vox::readers::obj::ObjReader;
use threed2vox::to_schematic;

fn main() {
    let config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Error)
        .build();
    TermLogger::init(LevelFilter::Debug, config, TerminalMode::Mixed).unwrap();

    let config = Config {
        voxel_size: VoxelOption::MeshSize(20.0),
        data_version: 2566,
        input_path: "models/teapot.obj".to_string(),
        filename: "teapot".to_string(),
        block: "stone".to_string(),
        x_rot: 0.0,
        y_rot: 0.0,
        z_rot: 0.0,
        threads: 4,
        nbtify: Box::new(SchematicV2),
        reader: Box::new(ObjReader),
    };
    let blob = to_schematic(config).unwrap();
    println!("{}", blob.len_bytes());
}
