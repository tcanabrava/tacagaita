
extern crate gl;

use itertools::izip;

use crate::gl_program::GLProgram;
use crate::textures::*;
use crate::transformation::Transformation;

use std::ffi::{CString};

pub struct Geometry {
    // vao id.
    vao: gl::types::GLuint,
    // shader program id.
    program: GLProgram,
    textures: Vec<Texture>,
    idx_size: gl::types::GLint,
    matrix: Transformation,
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

    pub fn matrix_mut(&mut self) -> &mut Transformation {
        return &mut self.matrix;
    }

    pub fn before_draw(&mut self) {
        let ptr = self.matrix_mut().internal_ptr();
        self.program_mut().set_matrix("model", ptr);
    }

    pub fn draw(&self) {
        // Activate loads the uniforms, so we need to set the uniform before activating the program.
        self.program().activate();
        unsafe {
            let mut curr_pos: i32 = 0;
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
        data : &[f32],
        indexes: &[i32],
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
            matrix: Transformation::new(),
        };
    }
}