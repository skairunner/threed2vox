

use std::env;

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
\
The largest difference between versions is pre- and post-1.13 (1241 vs 1626): the two use different schematic formats.")
        )
        .get_matches_from(env::args());

    to_schematic(AppConfig::from_argmatch(matches)?);

    Ok(())
}