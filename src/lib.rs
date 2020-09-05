pub mod voxel_grid;
pub mod nbtifier;
pub mod config;

use std::collections::HashMap;

use ncollide3d::na::{Isometry3, Point3, Vector3};
use ncollide3d::query;
use ncollide3d::shape::{Cuboid, TriMesh};
use tobj::{load_obj, Model};
use ncollide3d::query::Proximity;

use config::{Config, VoxelOption};
use voxel_grid::VoxelGrid;

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
pub fn to_schematic(config: Config) -> anyhow::Result<()> {
    let obj = read_obj(&config.input_path)?;
    let mut trimesh = obj_to_trimesh(obj);
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
    let x = f32::ceil(extents.x / voxel_size) as i32;
    let y = f32::ceil(extents.y / voxel_size) as i32;
    let z = f32::ceil(extents.z / voxel_size) as i32;

    let mut grid = VoxelGrid::new(x, y, z);
    let voxel = Cuboid::new(Vector3::new(voxel_size / 2.0, voxel_size / 2.0, voxel_size / 2.0));

    // Iterate over voxels and do collision tests
    println!("{} {} {} {}", x, y, z, voxel_size);
    let mut n = 0;
    for i in 0..x {
        for j in 0..y {
            println!("i= {}  j = {}", i, j);
            for k in 0..z {
                let transform = Isometry3::translation((i as f32) * voxel_size, (j as f32) * voxel_size, (k as f32) * voxel_size);
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
    println!("Voxels: {}", n);

    // Return

    Ok(())
}