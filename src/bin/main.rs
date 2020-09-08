

use std::env;
use std::fs::File;
use std::path::Path;

use clap::{Arg, App};
use threed2vox::to_schematic;
use threed2vox::config::{VoxelOption, Config as AppConfig};
use simplelog::*;

fn main() -> anyhow::Result<()> {
    SimpleLogger::new(LevelFilter::Debug, Config::default());

    let matches = App::new("threed2vox")
        .version("1.0")
        .author("Sky")
        .about("Converts 3D files to Minecraft .schematic format")
        .arg(Arg::with_name("input")
            .help("Designate the input file")
            .index(1)
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Designate the output file")
                .takes_value(true))
        .arg(Arg::with_name("max_size")
            .short("s")
            .long("size")
            .help("Designate how many blocks long the longest axis of the model is. Defaults to using 1 model unit = 1 block, and is overridden by --scale.")
            .overrides_with("scale")
            .takes_value(true)
        )
        .arg(Arg::with_name("scale")
            .short("S")
            .long("scale")
            .help("Specify a units-to-blocks ratio. Defaults to 1, and is overridden by --size.")
            .takes_value(true)
            .overrides_with("max_size")
        )
        .arg(Arg::with_name("block")
            .short("b")
            .long("block")
            .help("The block id string to use for the shell of the model. Defaults to stone.")
        )
        .arg(Arg::with_name("minecraft version")
            .short("V")
            .long("version")
            .help("Either the version (as a string) or the dataversion of Minecraft to make the schematic for.\
Refer to the minecraft_versions.toml file for the dataversions, or simply specify a version name and let threed2vox guess the dataversion for you.\
Another alternative is to specify 'none' version, though this is undefined behaviour. Note that threed2vox only supports Java Edition.\
\
The largest difference between versions is pre- and post-1.13 (1241 vs 1626): the two use different schematic formats.")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("x_rot")
            .short("x")
            .help("Each -x specified rotates the model on the x axis by 90 degrees.")
            .multiple(true)
            .takes_value(false)
        )
        .arg(Arg::with_name("y_rot")
            .short("y")
            .help("Each -y specified rotates the model on the y axis by 90 degrees.")
            .multiple(true)
            .takes_value(false)
        )
        .arg(Arg::with_name("z_rot")
            .short("z")
            .help("Each -z specified rotates the model on the z axis by 90 degrees.")
            .multiple(true)
            .takes_value(false)
        )
        .get_matches_from(env::args());

    let config = AppConfig::from_argmatch(matches)?;
    let input_path = config.input_path.clone();
    let path = Path::new(&input_path).parent().unwrap();
    let file_stem = config.filename.clone();

    let nbt = to_schematic(config)?;

    // Output nbt to file.
    let output_path = path.join(Path::new(&format!("{}.schematic", file_stem)));
    println!("[INFO] Writing to '{}'", output_path.to_str().unwrap());

    let mut file = File::create(output_path.clone())
        .expect(&format!("Could not create file '{:?}'", output_path));

    nbt.to_gzip_writer(&mut file);

    Ok(())
}