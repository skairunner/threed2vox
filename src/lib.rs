use std::collections::HashMap;

use ncollide3d::na::{Isometry3, Point3, Vector3};
use ncollide3d::query;
use ncollide3d::query::Proximity;
use ncollide3d::shape::{Cuboid, TriMesh};
use tobj::{load_obj, Model};

use config::{Config, VoxelOption};
use nbtifier::SchematicV2;
use voxel_grid::VoxelGrid;

use crate::nbtifier::NBTIfy;

pub mod voxel_grid;
pub mod nbtifier;
pub mod config;

pub fn read_obj(path: &str) -> anyhow::Result<Vec<Model>> {
    let (model, _) = load_obj(path, true)?;
    Ok(model)
}

/// Convert the output of tobj into one big trimesh
fn obj_to_trimesh(objs: Vec<Model>) -> TriMesh<f32> {
    let mut points: Vec<Point3<f32>> = vec!();
    let mut indices: Vec<Point3<usize>> = vec!();

    for obj in objs.into_iter() {
        let mesh = obj.mesh;

        let mut i = 0;
        while i <= mesh.indices.len() - 3 {
            let i1 = mesh.indices[i]   as usize;
            let i2 = mesh.indices[i+1] as usize;
            let i3 = mesh.indices[i+2] as usize;
            indices.push(Point3::from([i1, i2, i3]));
            i += 3;
        }

        let mut i = 0;
        while i <= mesh.positions.len() - 3 {
            let p1 = mesh.positions[i];
            let p2 = mesh.positions[i+1];
            let p3 = mesh.positions[i+2];
            points.push(Point3::from([p1, p2, p3]));
            i += 3;
        }
    }

    TriMesh::new(points, indices, None)
}

/// Read object from path and step through it with a given voxel size.
pub fn to_schematic(config: Config) -> anyhow::Result<nbt::Blob> {
    let obj = read_obj(&config.input_path)?;
    let mut trimesh = obj_to_trimesh(obj);

    trimesh.transform_by(
        &Isometry3::rotation(
            Vector3::new(config.x_rot, config.y_rot, config.z_rot)
        )
    );

    let mins = trimesh.aabb().mins;
    trimesh.transform_by(&Isometry3::translation(-mins.x, -mins.y, -mins.z));


    let voxel_size = match config.voxel_size {
        VoxelOption::VoxelSize(s) => s,
        VoxelOption::MeshSize(s) => {
            let aabb = trimesh.aabb();
            let extents = aabb.extents();
            let longest = if extents.x > extents.y { extents.x } else { extents.y };
            let longest = if longest > extents.z { longest } else { extents.z };

            longest / s
        }
    };

    // Determine the voxel grid size
    let aabb = trimesh.aabb();
    let extents = aabb.extents();
    let x = f32::ceil(extents.x / voxel_size) as i32 + 1;
    let y = f32::ceil(extents.y / voxel_size) as i32 + 1;
    let z = f32::ceil(extents.z / voxel_size) as i32 + 1;

    let mut grid = VoxelGrid::new(x, y, z);
    let voxel_half = voxel_size / 2.0;
    let voxel = Cuboid::new(Vector3::new(voxel_half, voxel_half, voxel_half));

    // Iterate over voxels and do collision tests
    println!("[INFO] Dimensions of the model are {}x{}x{}", x, y, z);
    let mut n = 0;
    for i in 0..x {
        println!("[INFO] {:.1}%", (i as f32) / (x as f32) * 100.0);
        for j in 0..y {
            for k in 0..z {
                let transform = Isometry3::translation(
                    (i as f32) * voxel_size - voxel_half, (j as f32) * voxel_size - voxel_half, (k as f32) * voxel_size - voxel_half);
                let proximity = query::proximity(&transform, &voxel, &Isometry3::translation(0.0, 0.0, 0.0), &trimesh, 0.0);
                match proximity {
                    Proximity::Intersecting => {
                        grid.set(i, j, k, true);
                        n += 1;
                    },
                    _ => ()
                }
            }
        }
    }
    println!("[INFO] Total {} blocks", n);

    // // debug print
    // for k in 0..z {
    //     for i in 0..x {
    //         let mut output = String::new();
    //         for j in 0..y {
    //             let c = match grid.get(i, j, k) {
    //                 false => '0',
    //                 true => '1'
    //             };
    //             output.push(c);
    //         }
    //         println!("{}", output);
    //     }
    //     println!();
    // }

    Ok(SchematicV2::convert(&grid, &config))
}