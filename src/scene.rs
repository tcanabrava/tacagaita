use crate::geometry::*;
use crate::transformation::Transformation;

pub struct Scene {
    geometries: Vec<Geometry>,
    matrix: Transformation,
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            geometries: Vec::new(),
            matrix: Transformation::new(),
        }
    }

    pub fn add_geometries(&mut self, geometries: Vec<Geometry>) {
        self.geometries = geometries;
    }

    pub fn matrix_mut(&mut self) -> &mut Transformation {
        return &mut self.matrix;
    }

    pub fn render(&self) {
        for element in &self.geometries {
            element.draw();
        }
    }
}
