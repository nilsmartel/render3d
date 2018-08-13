
use lina;

pub struct Mesh {
    vertecise: Vec<lina::Vector3>,
    normals: Vec<lina::Vector3>,
    uv_coordinates: Vec<lina::Vector2>,
    faces: Vec<FaceIDs>
}

struct FaceIDs {
    vert_id: usize,
    normal_id: usize,
    uv_id: usize
}

pub struct Face {
    pub vertex: lina::Vector3,
    pub normal: lina::Vector3,
    pub uv_coordinate: lina::Vector2
}

impl Face {
    pub fn new(
        vertex: lina::Vector3,
        normal: lina::Vector3,
        uv_coordinate: lina::Vector2
        ) -> Face { Face {vertex, normal, uv_coordinate} }   
}
 
impl<'a> IntoIterator for Mesh {
    type IntoIter = MeshIterator<'a>;
    type Item = Face;

    fn into_iter(&'a self) -> MeshIterator<'a> {
        MeshIterator {
            mesh: self,
            current: 0
        }
    }
}

pub struct MeshIterator<'a> {
    mesh: &'a Mesh,
    current: usize
}

impl<'a> Iterator for MeshIterator<'a> {
    type Item = Face;

    fn next(&mut self) -> Option<Face> {
        if self.current < self.mesh.faces.len() {
            let ids = &self.mesh.faces[self.current];
            self.current += 1;
            
            return Some(Face::new(
                self.mesh.vertecise[ids.vert_id],
                self.mesh.normals[ids.normal_id],
                self.mesh.uv_coordinates[ids.uv_id]
            ));
        }

        None
    }
}