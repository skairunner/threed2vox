use crate::readers::reader::Reader;
use anyhow::Error;
use nalgebra::Point3;
use parry3d::shape::TriMesh;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::OpenOptions;

pub struct StlReader;

impl Reader for StlReader {
    fn load(&self, path: &str) -> Result<TriMesh, Error> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let reader = stl_io::create_stl_reader(&mut file)?;
        // Store whether a vertex exists and if yes, which index
        let mut vertex_lookup = BTreeMap::new();
        // The vertex list
        let mut vertices = Vec::new();
        // The faces.
        let mut faces = Vec::new();

        for tri in reader {
            if let Ok(tri) = tri {
                let mut face: Vec<u32> = Vec::new();
                for vertex in &tri.vertices {
                    let point = (
                        vertex[0].to_bits(),
                        vertex[1].to_bits(),
                        vertex[2].to_bits(),
                    );
                    let index = vertex_lookup.entry(point).or_insert_with(|| {
                        vertices.push(Point3::new(vertex[0], vertex[1], vertex[2]));
                        vertices.len() - 1
                    });
                    face.push(*index as u32)
                }
                faces.push(face[0..3].try_into().unwrap());
            }
        }

        Ok(TriMesh::new(vertices, faces))
    }
}
