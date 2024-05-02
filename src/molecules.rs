use crate::model;

trait Molecule {}

// #[derive(Clone, Debug)]
// pub struct LineWater {
//     pub H1: model::Line,
//     pub H2: model::Line,
//     pub  O: model::Line,
// }

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct LineWater {
    pub H1_O_H2: model::Line,
}

impl LineWater {
    pub fn gen_vertex_index(&self) -> (Vec<model::Vertex>, Vec<model::Index>) {
        (self.H1_O_H2.vertices.clone(), self.H1_O_H2.indices.clone())
    }
}

impl Default for LineWater {
    fn default() -> Self {
        Self {
            H1_O_H2: model::Line::new(
                [0.0, 0.5, 1.0],
                vec![
                    [1.620, 1.997, 1.470], // H1
                    [1.530, 2.022, 1.486], // O
                    [1.526, 2.115, 1.463], // H2
                ],
            ),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct SphereWater {
    pub H1: model::Sphere,
    pub H2: model::Sphere,
    pub O: model::Sphere,
}

impl Molecule for SphereWater {}

impl SphereWater {
    pub fn gen_vertex_index(&self) -> (Vec<Vec<model::Vertex>>, Vec<Vec<model::Index>>) {
        let mut vertices = Vec::with_capacity(3);
        let mut indices = Vec::with_capacity(3);
        vertices.push(self.H1.vertices.clone());
        vertices.push(self.H2.vertices.clone());
        vertices.push(self.O.vertices.clone());
        indices.push(self.H1.indices.clone());
        indices.push(self.H2.indices.clone());
        indices.push(self.O.indices.clone());
        (vertices, indices)
    }
}

impl Default for SphereWater {
    fn default() -> Self {
        Self {
            H1: model::Sphere::new(0.01, [1.620, 1.997, 1.470], [0.0, 0.5, 1.0], 20),
            H2: model::Sphere::new(0.01, [1.526, 2.115, 1.463], [0.0, 0.5, 1.0], 20),
            O: model::Sphere::new(0.03, [1.530, 2.022, 1.486], [0.0, 0.5, 1.0], 20),
        }
    }
}
