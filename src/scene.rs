use crate::geometry::*;
use crate::transformation::Transformation;

pub struct Scene<'a> {
    geometries: Vec<Geometry<'a>>,
    projection_matrix: Transformation,
    view_matrix: Transformation,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        return Scene {
            geometries: Vec::new(),
            projection_matrix: Transformation::new(),
            view_matrix: Transformation::new(),
        };
    }

    pub fn projection(&mut self) -> &mut Transformation {
        return &mut self.projection_matrix;
    }

    pub fn view(&mut self) -> &mut Transformation {
        return &mut self.view_matrix;
    }

    pub fn geometries(&mut self) -> &mut Vec<Geometry<'a>> {
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
