use std::collections::HashMap;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    let file = std::fs::File::open(&args[1])?;
    let schem: HashMap<String, nbt::Value> = nbt::from_gzip_reader(file)?;

    for (key, value) in schem {
        println!("{:?}: {:?}", key, value.tag_name());
    }

    Ok(())
}
