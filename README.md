# threed2vox

A Rust program to convert from 3D models (.obj, .stl) to Minecraft schematics (.schem). Takes full advantage of parallel cores to speed up generation.

Can output WorldEdit Schematics (`.schem`) and Structure Format (`.nbt`), and any block can be output.

## How to Run
Clone the repo and run something like:

```
cargo run -- --output schematics/ --size 50 --block minecraft:stone --version 1.16 -x my_input.obj
```

Make sure that the input file is at the end. The file format is detected from the file extension.

### Supported file formats
|Extensions|Format|
|----------|------|
|`.obj`|Waveform object file|
|`.stl`|STL format|
|`.dae`|COLLADA exchange format|

## Arguments

|Long|Short|Description|
|----|-----|-----------|
|`--output`|`-o`|Designate the output path|
|`--size`|`-s`|Designate how many blocks long the longest axis of the model is. Default is 1 model unit = 1 block.|
|`--scale`|`-S`|Specify a units-to-blocks ratio. Defaults to 1.|
|`--block`|`-b`|Specify what block the shell of the model will be. Defaults to stone.|
|`--version`|`-V`|Specify the version of minecraft for which to output for. Currently only supports 1.13+|
|`--format`|`-f`|Specify the format to output in. Valid options are "schem", "schematic" for schematic files, and "nbt", "structure" for Structure files. Defaults to Schematic.|
||`-x`|Rotate the model by 90 degrees on the X axis. Can specify multiple times, e.g. `-xx`|
||`-y`|Rotate the model by 90 degrees on the Y axis. Can specify multiple times.|
||`-z`|Rotate the model by 90 degrees on the Z axis. Can specify multiple times.|
|`--threads`|`-t`|Manually specify the number of threads to use. Shouldn't be necessary, as it defaults to the number of physical cores available minus one.|


