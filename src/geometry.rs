
extern crate gl;

use crate::shader::*;

pub struct DataBlock {
    pub size: i32,
    pub offset: u32
}

pub struct Geometry {
    // vao id.
    vao: gl::types::GLuint,
    // shader program id.
    program: GLProgram,
}

impl Geometry {
    pub fn vao(&self) -> gl::types::GLuint {
        return self.vao;
    }

    pub fn program(&self) -> &GLProgram {
        return &self.program;
    }

    pub fn from_data(data : &Vec<f32>, indexes: &Vec<i32>, program_id: GLProgram, data_block: &[&DataBlock]) -> Geometry {
        let mut vbo: gl::types::GLuint = 0;
        let mut vao: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;

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

            gl::EnableVertexAttribArray(0);
            let mut idx = 0;
            for dblock in data_block {
                println!("Adding attrib pointer {:?}", idx);
                gl::VertexAttribPointer(
                    idx,                                      // Index of the array.
                    3,                                      // number of points to consider inside of the array.
                    gl::FLOAT,                              // type of the data
                    gl::FALSE,                              // Dados tem que ser normalizados? (entre -1.0f e 1.0f)
                    dblock.size * std::mem::size_of::<f32>() as gl::types::GLint,  // size of each "block" of data
                    dblock.offset as *const std::ffi::c_void  // where the data begins, inside of the array
                );
                idx += 1;
            }
            gl::BindVertexArray(0);
        };

        return Geometry{vao: vao, program: program_id};
    }
}