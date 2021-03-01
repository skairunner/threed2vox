use crate::readers::reader::Reader;
use anyhow::Error;
use nalgebra::Point3;
use parry3d::shape::TriMesh;
use std::collections::BTreeMap;

pub struct GltfReader;

impl Reader for GltfReader {
    fn load(&self, path: &str) -> Result<TriMesh, Error> {
        let (gltf, buffers, _) = gltf::import(path)?;
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        let mut vertices: Vec<Point3<f32>> = Vec::new();
        // Store whether a vertex exists and if yes, which index
        // let mut vertex_lookup = BTreeMap::new();
        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    let positions: Vec<_> = iter.collect();
                    for face in positions.chunks(3) {
                        // let mut buffer = Vec::new();
                        // for pos in face {
                        //     let pos_bits = (pos[0].to_bits(), pos[1].to_bits(), pos[2].to_bits());
                        //     let index = vertex_lookup.entry(pos_bits)
                        //         .or_insert_with(|| {
                        //             vertices.push(Point3::new(pos[0], pos[1], pos[2]));
                        //             vertices.len() - 1
                        //         });
                        //     buffer.push(*index);
                        // }
                        let i = vertices.len();
                        for pos in face {
                            vertices.push(Point3::new(pos[0], pos[1], pos[2]));
                        }
                        triangles.push([i as u32, (i + 1) as u32, (i + 2) as u32]);
                    }
                }
            }
        }

        Ok(TriMesh::new(vertices, triangles))
    }
}
