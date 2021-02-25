use crate::readers::reader::Reader;
use parry3d::na::Point3;
use parry3d::shape::TriMesh;
use tobj::{load_obj, Model};

pub struct ObjReader;

impl Reader for ObjReader {
    fn load(&self, path: &str) -> Result<TriMesh, anyhow::Error> {
        let model = read_obj(path)?;
        Ok(obj_to_trimesh(model))
    }
}

fn read_obj(path: &str) -> anyhow::Result<Vec<Model>> {
    let (model, _) = load_obj(path, true).map_err(|e| {
        log::error!("Could not open file {}: {:?}", path, e);
        e
    })?;
    Ok(model)
}

/// Convert the output of tobj into one big trimesh
fn obj_to_trimesh(objs: Vec<Model>) -> TriMesh {
    let mut points: Vec<Point3<f32>> = vec![];
    let mut indices: Vec<[u32; 3]> = vec![];

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
