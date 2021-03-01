use crate::readers::reader::Reader;
use anyhow::Error;
use collada::document::ColladaDocument;
use nalgebra::Point3;
use parry3d::shape::TriMesh;
use std::path::Path;

/// The Collada reader.
/// If multiple polys are found, they are all mushed together.
pub struct DaeReader;

impl Reader for DaeReader {
    fn load(&self, path: &str) -> Result<TriMesh, Error> {
        let doc =
            ColladaDocument::from_path(&Path::new(path)).map_err(|s| anyhow::anyhow!("{}", s))?;
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        let mut vertices: Vec<Point3<f32>> = Vec::new();

        if let Some(objset) = doc.get_obj_set() {
            let offset = vertices.len() as u32;
            for object in objset.objects {
                object
                    .vertices
                    .iter()
                    .for_each(|v| vertices.push(Point3::new(v.x as f32, v.y as f32, v.z as f32)));
                object.geometry.iter()
                    .flat_map(|g| &g.mesh)
                    .for_each(|el| match el {
                        collada::PrimitiveElement::Polylist(_) => panic!("Discovered a polylist in the collada file. Polylists are not supported."),
                        collada::PrimitiveElement::Triangles(tris) => {
                            for tri in &tris.vertices {
                                triangles.push([tri.0 as u32 + offset, tri.1 as u32 + offset, tri.2 as u32 + offset]);
                            }
                        }
                    });
            }

            Ok(TriMesh::new(vertices, triangles))
        } else {
            Err(anyhow::anyhow!("Did not find object set"))
        }
    }
}
