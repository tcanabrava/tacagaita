use crate::geometry::*;
use crate::transformation::Transformation;
use crate::gl_program::GLProgram;

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

    pub fn geometries(&mut self, ) -> &mut Vec<Geometry> {
        return &mut self.geometries;
    }

    pub fn render(&mut self) {
        // Apply transformations here.

        for element in &mut self.geometries {
            let program = element.program_mut();
            program.set_matrix("projection", self.projection_matrix.internal_ptr());
            program.set_matrix("view", self.view_matrix.internal_ptr());

            element.before_draw();
            element.draw();
        }
    }

}
