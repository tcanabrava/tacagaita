use crate::geometry::*;
use crate::transformation::Transformation;

pub struct Scene {
    geometries: Vec<Geometry>,
    projection_matrix: Transformation,
    view_matrix: Transformation,
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            geometries: Vec::new(),
            projection_matrix: Transformation::new(),
            view_matrix: Transformation::new(),
        }
    }

    pub fn add_geometries(&mut self, geometries: Vec<Geometry>) {
        self.geometries = geometries;
    }

    pub fn render(&mut self) {
        // Apply transformations here.

        for element in &mut self.geometries {
            element.before_draw();
            element.draw();
        }
    }

}
