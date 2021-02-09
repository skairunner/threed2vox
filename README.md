# threed2vox

A Rust program to convert from 3D models (.obj) to Minecraft schematics (.schem). Takes full advantage of parallel cores to speed up generation.

Supports WorldEdit schematic format, and any block to be output. Though Structure format (aka .nbt) is possible, it may or may not happen depending on the author's needs.

## How to Run
Clone the repo and run:

```
cargo run -- --output schematics/ --size 50 --block create:granite --version 1.16 -x my_input.obj
```


|Long|Short|Description|
|----|-----|-----------|
|`--output`|`-o`|Designate the output path|
|`--size`|`-s`|Designate how many blocks long the longest axis of the model is. Default is 1 model unit = 1 block.|
|`--scale`|`-S`|Specify a units-to-blocks ratio. Defaults to 1.|
|`--block`|`-b`|Specify what block the shell of the model will be. Defaults to stone.|
|`--version`|`-V`|Specify the version of minecraft for which to output for. Currently only supports 1.13+|
||`-x`|Rotate the model by 90 degrees on the X axis. Can specify multiple times, e.g. `-xx`|
||`-y`|Rotate the model by 90 degrees on the Y axis. Can specify multiple times.|
||`-z`|Rotate the model by 90 degrees on the Z axis. Can specify multiple times.|


