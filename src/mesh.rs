use lina;

type Vector3 = lina::Vector3<f32>;

pub struct Mesh {
    vertecise: Vec<Vector3>,
    normals: Vec<Vector3>,
    uv_coordinates: Vec<lina::Vector2<f32>>,
    faces: Vec<FaceIDs>,
}

impl Mesh {
    fn new(
        vertecise: Vec<Vector3>,
        normals: Vec<Vector3>,
        uv_coordinates: Vec<lina::Vector2<f32>>,
        faces: Vec<FaceIDs>,
    ) -> Mesh {
        Mesh {
            vertecise,
            normals,
            uv_coordinates,
            faces,
        }
    }

    pub fn transform(&self, matrix: &lina::Matrix3x3<f32>) -> Mesh {
        Mesh::new(
            self.vertecise.iter().map(|v| matrix * *v).collect(),
            self.normals.iter().map(|v| matrix * *v).collect(),
            self.uv_coordinates.clone(),
            self.faces.clone(),
        )
    }
}

#[derive(Clone)]
struct FaceIDs {
    vert_id: usize,
    normal_id: usize,
    uv_id: usize,
}

pub struct Face {
    pub vertex: Vector3,
    pub normal: Vector3,
    pub uv_coordinate: lina::Vector2<f32>,
}

impl Face {
    pub fn new(vertex: Vector3, normal: Vector3, uv_coordinate: lina::Vector2<f32>) -> Face {
        Face {
            vertex,
            normal,
            uv_coordinate,
        }
    }
}

impl<'a> IntoIterator for &'a Mesh {
    type IntoIter = MeshIterator<'a>;
    type Item = Face;

    fn into_iter(self) -> MeshIterator<'a> {
        MeshIterator {
            mesh: self,
            current: 0,
        }
    }
}

pub struct MeshIterator<'a> {
    mesh: &'a Mesh,
    current: usize,
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
                self.mesh.uv_coordinates[ids.uv_id],
            ));
        }

        None
    }
}
