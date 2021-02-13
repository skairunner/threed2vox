use tobj::{load_obj, Model};

use config::{Config, VoxelOption};
use nbtifier::SchematicV2;
use voxel_grid::VoxelGrid;

use crate::nbtifier::NBTIfy;
use rayon::prelude::*;
use std::sync::Mutex;
use nalgebra::{Vector3, Translation3};
use parry3d::shape::{TriMesh, Cuboid};
use parry3d::na::{Point3, Isometry3};
use parry3d::query;
use parry3d::motion::RigidMotionComposition;

pub mod config;
pub mod nbtifier;
pub mod voxel_grid;

pub fn read_obj(path: &str) -> anyhow::Result<Vec<Model>> {
    let (model, _) = load_obj(path, true).map_err(|e| {
        log::error!("Could not open file {}: {:?}", path, e);
        e
    })?;
    Ok(model)
}

/// Convert the output of tobj into one big trimesh
fn obj_to_trimesh(objs: Vec<Model>) -> TriMesh {
    let mut points: Vec<Point3<f32>> = vec![];
    let mut indices: Vec<[u32;3]> = vec![];

    for obj in objs.into_iter() {
        let mesh = obj.mesh;

        let mut i: usize = 0;
        while i as i32 <= (mesh.indices.len() as i32) - 3 {
            let i1 = mesh.indices[i];
            let i2 = mesh.indices[i + 1];
            let i3 = mesh.indices[i + 2];
            indices.push([i1, i2, i3]);
            i += 3;
        }

        let mut i: usize = 0;
        while i as i32 <= (mesh.positions.len() as i32) - 3 {
            let p1 = mesh.positions[i];
            let p2 = mesh.positions[i + 1];
            let p3 = mesh.positions[i + 2];
            points.push(Point3::from([p1, p2, p3]));
            i += 3;
        }
    }

    TriMesh::new(points, indices)
}

/// Read object from path and step through it with a given voxel size.
pub fn to_schematic(config: Config) -> anyhow::Result<nbt::Blob> {
    log::info!("Loading model.");
    let obj = read_obj(&config.input_path)?;
    let trimesh = obj_to_trimesh(obj);

    let mut trimesh_transform = Isometry3::rotation(Vector3::new(
        config.x_rot,
        config.y_rot,
        config.z_rot,
    ));
    let mins = trimesh.aabb(&trimesh_transform).mins;
    trimesh_transform.translation = Translation3::new(-mins.x, -mins.y, -mins.z);

    let voxel_size = match config.voxel_size {
        VoxelOption::VoxelSize(s) => s,
        VoxelOption::MeshSize(s) => {
            let aabb = trimesh.aabb(&trimesh_transform);
            let extents = aabb.extents();
            let longest = if extents.x > extents.y {
                extents.x
            } else {
                extents.y
            };
            let longest = if longest > extents.z {
                longest
            } else {
                extents.z
            };

            longest / s
        }
    };

    // Determine the voxel grid size
    let aabb = trimesh.aabb(&trimesh_transform);
    let extents = aabb.extents();
    let x = f32::ceil(extents.x / voxel_size) as i32 + 1;
    let y = f32::ceil(extents.y / voxel_size) as i32 + 1;
    let z = f32::ceil(extents.z / voxel_size) as i32 + 1;

    let mut grid = VoxelGrid::new(x, y, z);

    // Iterate over voxels and do collision tests
    log::info!(
        "Dimensions of the model are {}x{}x{}. Starting conversion.",
        x,
        y,
        z
    );

    let results = match cfg!(feature = "sequential") {
        true => do_collision_seq((x, y, z), voxel_size, &trimesh, &trimesh_transform),
        false => do_collision_par((x, y, z), voxel_size, &trimesh, &trimesh_transform),
    };

    results
        .into_iter()
        .for_each(|(i, j, k)| grid.set(i, j, k, true));

    Ok(SchematicV2::convert(&grid, &config).unwrap())
}

/// The inner part of do_collision_*
fn actually_do_collision(
    xyz: (i32, i32, i32),
    voxel_size: f32,
    voxel: &Cuboid,
    trimesh: &TriMesh,
    pos: &Isometry3<f32>,
) -> Option<(i32, i32, i32)> {
    let (x, y, z) = xyz;
    let voxel_half = voxel_size / 2.0;

    let transform = Isometry3::translation(
        (x as f32) * voxel_size - voxel_half,
        (y as f32) * voxel_size - voxel_half,
        (z as f32) * voxel_size - voxel_half,
    );
    let proximity = query::contact(
        &transform,
        voxel,
        pos,
        trimesh,
        0.0,
    ).unwrap();
    match proximity {
        Some(_) => Some((x, y, z)),
        None => None,
    }
}

fn do_collision_par(
    xyz: (i32, i32, i32),
    voxel_size: f32,
    trimesh: &TriMesh,
    pos: &Isometry3<f32>,
) -> Vec<(i32, i32, i32)> {
    let (x, y, z) = xyz;
    let voxel_half = voxel_size / 2.0;
    let voxel = Cuboid::new(Vector3::new(voxel_half, voxel_half, voxel_half));

    let progress = Mutex::new(0);
    (0..x)
        .into_par_iter()
        .map(|i| {
            let result = (0..y)
                .into_par_iter()
                .map(|j| {
                    (0..z)
                        .into_par_iter()
                        .filter_map(|k| actually_do_collision((i, j, k), voxel_size, &voxel, trimesh, pos))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();
            let mut p = progress.lock().unwrap();
            *p += 1;
            log::info!("Progress {:.2}%", (*p as f32) / (x as f32) * 100.0);
            result
        })
        .flatten()
        .collect()
}

fn do_collision_seq(
    xyz: (i32, i32, i32),
    voxel_size: f32,
    trimesh: &TriMesh,
    pos: &Isometry3<f32>,
) -> Vec<(i32, i32, i32)> {
    let (x, y, z) = xyz;
    let voxel_half = voxel_size / 2.0;
    let voxel = Cuboid::new(Vector3::new(voxel_half, voxel_half, voxel_half));

    (0..x)
        .into_iter()
        .map(|i| {
            let result = (0..y)
                .into_iter()
                .map(|j| {
                    (0..z)
                        .into_iter()
                        .filter_map(|k| actually_do_collision((i, j, k), voxel_size, &voxel, trimesh, pos))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();
            log::info!("Progress {:.2}%", (i as f32) / (x as f32) * 100.0);
            result
        })
        .flatten()
        .collect()
}
