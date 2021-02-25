use config::{Config, VoxelOption};
use voxel_grid::VoxelGrid;

use nalgebra::{Translation3, Vector3};
use parry3d::na::Isometry3;
use parry3d::query::{
    ContactManifold, ContactManifoldsWorkspace, DefaultQueryDispatcher, PersistentQueryDispatcher,
};
use parry3d::shape::{Cuboid, TriMesh};
use rayon::prelude::*;
use std::sync::Mutex;

pub mod config;
mod nbt_helper;
pub mod nbtifier;
pub mod readers;
pub mod voxel_grid;

/// Read object from path and step through it with a given voxel size.
pub fn to_schematic(config: Config) -> anyhow::Result<nbt::Blob> {
    log::info!("Loading model.");
    let trimesh = config.reader.load(&config.input_path)?;

    let mut trimesh_transform =
        Isometry3::rotation(Vector3::new(config.x_rot, config.y_rot, config.z_rot));
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

    Ok(config.nbtify.convert(&grid, &config).unwrap())
}

/// The inner part of do_collision_*
fn actually_do_collision(
    xyz: (i32, i32, i32),
    voxel_size: f32,
    voxel: &Cuboid,
    trimesh: &TriMesh,
    pos: &Isometry3<f32>,
    manifold: &mut Vec<ContactManifold<(), ()>>,
    workspace: &mut Option<ContactManifoldsWorkspace>,
) -> Option<(i32, i32, i32)> {
    let (x, y, z) = xyz;
    let voxel_half = voxel_size / 2.0;

    let transform = Isometry3::translation(
        (x as f32) * voxel_size - voxel_half,
        (y as f32) * voxel_size - voxel_half,
        (z as f32) * voxel_size - voxel_half,
    );

    let dispatch = DefaultQueryDispatcher;

    // See source code for contact_shape_shape for why
    let pos12 = transform.inv_mul(pos);

    dispatch
        .contact_manifolds(&pos12, voxel, trimesh, 0.0, manifold, workspace)
        .unwrap();

    match manifold.len() {
        0 => None,
        _ => Some((x, y, z)),
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
                    let mut range = (0..z).peekable();
                    let mut z_range = Vec::new();
                    while range.peek().is_some() {
                        let chunk: Vec<_> = range.by_ref().take(100).collect();
                        z_range.push(chunk);
                    }
                    z_range
                        .into_par_iter()
                        .map(|ks| {
                            let mut manifolds = Vec::new();
                            let mut workspace = None;
                            let mut output = Vec::new();
                            for k in ks {
                                if let Some(o) = actually_do_collision(
                                    (i, j, k),
                                    voxel_size,
                                    &voxel,
                                    trimesh,
                                    pos,
                                    &mut manifolds,
                                    &mut workspace,
                                ) {
                                    output.push(o);
                                }
                            }
                            output
                        })
                        .flatten()
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

    let mut manifold = Vec::new();
    let mut workspace = None;

    (0..x)
        .map(|i| {
            let result = (0..y)
                .map(|j| {
                    (0..z)
                        .filter_map(|k| {
                            actually_do_collision(
                                (i, j, k),
                                voxel_size,
                                &voxel,
                                trimesh,
                                pos,
                                &mut manifold,
                                &mut workspace,
                            )
                        })
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
