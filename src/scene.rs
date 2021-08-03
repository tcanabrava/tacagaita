use crate::{camera, geometry::*};
use crate::transformation::Transformation;
use crate::camera::Camera;

pub struct Scene<'a> {
    geometries: Vec<Geometry<'a>>,
    projection_matrix: Transformation,
    camera: Camera,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        return Scene {
            geometries: Vec::new(),
            projection_matrix: Transformation::new(),
            camera: Camera::new(),
        };
    }

    pub fn projection(&mut self) -> &mut Transformation {
        return &mut self.projection_matrix;
    }

    pub fn _camera(&mut self) -> &mut Camera {
        return &mut self.camera;
    }

    pub fn geometries(&mut self) -> &mut Vec<Geometry<'a>> {
        return &mut self.geometries;
    }

    pub fn render(&mut self) {
        // Apply transformations here.

        for element in &mut self.geometries {
            let view_matrix = self.camera.view();
            let program = element.program_mut();
            program.set_matrix("projection", self.projection_matrix.internal_ptr());
            program.set_matrix("view", view_matrix.as_slice().as_ptr());
            element.before_draw();
            element.draw();
        }
    }

    pub fn key_event(&mut self, key: glfw::Key, _scancode: i32, action: glfw::Action, _modifiers: glfw::Modifiers) -> bool {
        let used = self.camera.key_event(key, _scancode, action, _modifiers);
        return used;
    }

}
