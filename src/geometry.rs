
extern crate gl;

use itertools::izip;

use crate::shader::*;
use crate::textures::*;
use nalgebra::{Matrix4, Vector3};
use std::ffi::{CString};

pub struct Geometry {
    // vao id.
    vao: gl::types::GLuint,
    // shader program id.
    program: GLProgram,
    textures: Vec<Texture>,
    idx_size: gl::types::GLint,
    transformations: Matrix4<f32>,
}

impl Geometry {
    pub fn vao(&self) -> gl::types::GLuint {
        return self.vao;
    }

    pub fn program(&self) -> &GLProgram {
        return &self.program;
    }

    pub fn program_mut(&mut self) -> &mut GLProgram {
        return &mut self.program;
    }

    pub fn idx_size(&self) -> gl::types::GLint {
        return self.idx_size;
    }

    pub fn scale(&mut self, scale_factor: f32) {
        self.transformations.append_scaling_mut(scale_factor)
    }

    pub fn translade(&mut self, x: f32, y:f32, z:f32) {
        self.transformations.append_translation_mut(&Vector3::new(x,y,z))
    }

    pub fn reset_transformations(&mut self) {
        self.transformations = Matrix4::identity();
    }

    pub fn rotate(&mut self) {
        println!("Rotate: Not implemented yet");
        // Rotation Example:
        // let rot        = Matrix4::from_scaled_axis(&Vector3::x() * 3.14);
        // let rot_then_m = matrix * rot; // Right-multiplication is equivalent to prepending `rot` to `m`.
        // let m_then_rot = rot * matrix; // Left-multiplication is equivalent to appending `rot` to `m`.
    }

    pub fn draw(&self) {
        self.program().activate();

        // load transformation matrix:
        let transform_c_str = CString::new("transform").expect("seriously");
        let transform_loc = self.program.get_location(&transform_c_str)
            .expect("Error getting transform location, verify if it's used. the driver usually removes unused variables");

        unsafe {
            let mut curr_pos: i32 = 0;

            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, self.transformations.as_slice().as_ptr());
            for texture in self.textures.iter() {
                gl::ActiveTexture(gl::TEXTURE0 + (curr_pos as u32));
                gl::BindTexture(gl::TEXTURE_2D, texture.id());
                curr_pos += 1;
            }

            gl::BindVertexArray(self.vao());
            gl::DrawElements(gl::TRIANGLES, self.idx_size(), gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }

    pub fn from_data(
        data : &Vec<f32>,
        indexes: &Vec<i32>,
        mut program_id: GLProgram,
        textures: Vec<Texture>,
        data_size: i32,
        offsets: &[(i32, usize)]) -> Geometry {

        let mut vbo: gl::types::GLuint = 0;
        let mut vao: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;

        for (idx, tex) in izip!(0..textures.len(), &textures) {
            program_id.set_int(tex.uniform(), idx as i32);
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1,  &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indexes.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                indexes.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            let mut idx = 0;
            for (size,offset) in offsets {
                println!("Adding attrib pointer {:?}, dblock: {:?}", idx, offset);
                gl::EnableVertexAttribArray(idx);
                gl::VertexAttribPointer(
                    idx,                                      // Index of the array.
                    *size,                                      // number of points to consider inside of the array.
                    gl::FLOAT,                              // type of the data
                    gl::FALSE,                              // Dados tem que ser normalizados (entre -1.0f e 1.0f)
                    data_size * std::mem::size_of::<f32>() as gl::types::GLint,  // size of each "block" of data
                    (offset * std::mem::size_of::<f32>()) as *const std::ffi::c_void  // where the data begins, inside of the array
                );
                idx += 1;
                gl::BindVertexArray(idx);
            }
        };

        return Geometry{
            vao: vao,
            program: program_id,
            textures: textures,
            idx_size: indexes.len() as gl::types::GLint,
            transformations: Matrix4::identity(),
        };
    }
}