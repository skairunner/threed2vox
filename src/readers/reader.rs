use parry3d::shape::TriMesh;

/// Used for types that can read from various 3D files and output a trimesh
pub trait Reader {
    /// Load a file and return a TriMesh
    fn load(&self, path: &str) -> Result<TriMesh, anyhow::Error>;
}
