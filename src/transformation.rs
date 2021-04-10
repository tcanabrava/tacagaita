use nalgebra::{Matrix4, Vector3};
use nalgebra_glm as glm;

extern crate gl;

pub type Angle = Vector3<f32>;

pub struct Transformation {
    transformations: Matrix4<f32>,
}

impl Transformation {
    // TODO: Move those away from here in the future in a way that we can share with Geometry.
    pub fn scale(&mut self, scale_factor: f32) {
        self.transformations.append_scaling_mut(scale_factor)
    }

    pub fn translade(&mut self, x: f32, y: f32, z: f32) {
        self.transformations
            .append_translation_mut(&Vector3::new(x, y, z))
    }

    pub fn reset_transformations(&mut self) {
        self.transformations = Matrix4::identity();
    }

    pub fn rotate(&mut self, angle: Angle) {
        let rot = Matrix4::from_scaled_axis(radians(angle));

        self.transformations = self.transformations * rot;
    }

    pub fn perspective(&mut self, aspect: f32, fovy: f32, znear: f32, zfar: f32) {
        self.transformations = glm::perspective(radians(aspect), fovy, znear, zfar);
    }

    pub fn new() -> Transformation {
        return Transformation {
            transformations: Matrix4::identity(),
        };
    }

    pub fn internal_ptr(&self) -> *const f32 {
        return self.transformations.as_slice().as_ptr();
    }

    pub fn set_uniform(&self) {}
}

pub fn radians<T>(degrees: T) -> T
where
    T: std::ops::Mul<f32, Output = T>
{
    const FRAC: f32 = std::f32::consts::PI / 180.0;
    return degrees * FRAC;
}
