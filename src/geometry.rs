
extern crate gl;
pub struct Geometry {
    vao: gl::types::GLuint,
}

impl Geometry {
    pub fn vao(&self) -> gl::types::GLuint {
        return self.vao;
    }

    pub fn from_data(data : &Vec<f32>, indexes: &Vec<i32>) -> Geometry {
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
            gl::VertexAttribPointer(
                0,                                      // Index of the array.
                3,                                      // number of points to consider inside of the array.
                gl::FLOAT,                              // type of the data
                gl::FALSE,                              // Dados tem que ser normalizados? (entre -1.0f e 1.0f)
                3 * std::mem::size_of::<f32>() as gl::types::GLint,  // size of each "block" of data
                std::ptr::null()                      // where the data begins, inside of the array
            );
            gl::BindVertexArray(0);
        };

        return Geometry{vao: vao};
    }
}