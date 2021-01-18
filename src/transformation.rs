use nalgebra::{Matrix4, Vector3};
use nalgebra_glm as glm;
use std::ffi::{CString};

extern crate gl;

pub enum Angle {
    X(f32),
    Y(f32),
    Z(f32)
}
pub struct Transformation {
    transformations: Matrix4<f32>,
}

impl Transformation {
    // TODO: Move those away from here in the future in a way that we can share with Geometry.
    pub fn scale(&mut self, scale_factor: f32) {
        self.transformations.append_scaling_mut(scale_factor)
    }

    pub fn translade(&mut self, x: f32, y:f32, z:f32) {
        self.transformations.append_translation_mut(&Vector3::new(x,y,z))
    }

    pub fn reset_transformations(&mut self) {
        self.transformations = Matrix4::identity();
    }

    pub fn rotate(&mut self, angle: Angle) {

        let rot = match angle {
            Angle::X(angle) => Matrix4::from_scaled_axis(&Vector3::x() * radians(angle)),
            Angle::Y(angle) => Matrix4::from_scaled_axis(&Vector3::y() * radians(angle)),
            Angle::Z(angle) => Matrix4::from_scaled_axis(&Vector3::z() * radians(angle))
        };

        self.transformations = self.transformations * rot;
    }

    pub fn perspactive(&mut self, aspect: f32, fovy: f32, znear: f32, zfar: f32) {
        self.transformations = glm::perspective( radians(aspect), fovy, znear, zfar);
    }

    pub fn new() -> Transformation {
        return Transformation {
            transformations: Matrix4::identity(),
        }
    }

    pub fn internal_ptr(&self) -> *const f32 {
        return self.transformations.as_slice().as_ptr();
    }

    pub fn set_uniform(&self) {

    }
}

pub fn radians(degrees: f32) -> f32 {
    const FRAC: f32 = std::f32::consts::PI / 180.0;
    return degrees * FRAC;
}
