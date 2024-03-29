extern crate gl;
use itertools::izip;
use rand::Rng;
use std::boxed::Box;

use crate::gl_program::GLProgram;
use crate::textures::*;
use crate::transformation::radians;
use crate::transformation::Transformation;
use crate::Shader;

pub struct Geometry<'a> {
    // vao id.
    vao: gl::types::GLuint,

    // shader program id.
    program: GLProgram,

    textures: Vec<Texture>,
    idx_size: gl::types::GLint,
    matrix: Transformation,

    // We are using many copies of this element, each one on this specified position.
    positions: Vec<nalgebra::Vector3<f32>>,

    // a lambda that we call from time to time.
    timer_func: Option<Box<dyn FnMut(&mut Transformation) -> () + 'a>>,
}

impl<'a> Geometry<'a> {
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

    pub fn set_positions(&mut self, positions: Vec<nalgebra::Vector3<f32>>) {
        self.positions = positions;
    }

    pub fn before_draw(&mut self) {
        let ptr = self.matrix_mut().internal_ptr();
        self.program_mut().set_matrix("model", ptr);
    }

    pub fn set_render_func<Func>(&mut self, callback: Func)
    where
        Func: 'a,
        Func: FnMut(&mut Transformation) -> (),
    {
        self.timer_func = Some(Box::new(callback));
    }

    pub fn draw(&mut self) {
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
            if let Some(timer_func) = &mut self.timer_func {
                timer_func(&mut self.matrix);
            }

            if self.positions.is_empty() {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.idx_size(),
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            } else {
                for position in self.positions.iter() {
                    let angle: f32 = radians(20.0 * rand::thread_rng().gen::<f32>());
                    let axis = glm::vec3(1.0, 0.3, 0.5);
                    let mut model = nalgebra::Matrix4::identity();

                    model = glm::translate(&model, &position);
                    model = glm::rotate(&model, radians(angle), &axis);
                    self.program.set_matrix("model", model.as_slice().as_ptr());

                    gl::DrawElements(
                        gl::TRIANGLES,
                        self.idx_size(),
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );
                }
            }
            gl::BindVertexArray(0);
        }
    }

    pub fn from_data(
        data: &[f32],
        indexes: &[i32],
        mut program_id: GLProgram,
        textures: Vec<Texture>,
        data_size: i32,
        offsets: &[(i32, usize)],
    ) -> Geometry<'a> {
        let mut vbo: gl::types::GLuint = 0;
        let mut vao: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;

        for (idx, tex) in izip!(0..textures.len(), &textures) {
            program_id.set_int(tex.uniform(), idx as i32);
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indexes.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                indexes.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            let mut idx = 0;
            for (size, offset) in offsets {
                #[cfg(debug_assertions)]
                println!("Adding attrib pointer {:?}, dblock: {:?}", idx, offset);
                gl::EnableVertexAttribArray(idx);
                gl::VertexAttribPointer(
                    idx,                                                              // Index of the array.
                    *size,     // number of points to consider inside of the array.
                    gl::FLOAT, // type of the data
                    gl::FALSE, // Dados tem que ser normalizados (entre -1.0f e 1.0f)
                    data_size * std::mem::size_of::<f32>() as gl::types::GLint, // size of each "block" of data
                    (offset * std::mem::size_of::<f32>()) as *const std::ffi::c_void, // where the data begins, inside of the array
                );
                idx += 1;
                gl::BindVertexArray(idx);
            }
        };

        return Geometry {
            vao: vao,
            program: program_id,
            textures: textures,
            idx_size: indexes.len() as gl::types::GLint,
            matrix: Transformation::new(),
            positions: Vec::new(),
            timer_func: None,
        };
    }

    pub fn rectangle(
        width: f32,
        height: f32,
        frag: &'a str,
        vert: &'a str,
    ) -> anyhow::Result<Geometry<'a>> {
        let triangle_frag = Shader::from_fragment_src(frag)?;
        let triangle_vert = Shader::from_vertex_src(vert)?;

        let gl_program_1 = GLProgram::from_shaders(&[&triangle_vert, &triangle_frag])?;

        // Create something to handle the coordinate conversions.
        let top: f32 = (height / 2.0) * 0.01;
        let bottom: f32 = -top;
        let right: f32 = (width / 2.0) * 0.01;
        let left: f32 = -right;

        #[rustfmt::skip]
        let data: Vec<f32> = vec![
            // vertices     |// Colors      // Texture
            top,    right, 0.0,  1.0, 0.0, 0.0,  1.0, 1.0, // top right     // 0
            bottom, right, 0.0,  0.0, 1.0, 0.0,  1.0, 0.0, // bottom right  // 1
            bottom, left,  0.0,  1.0, 0.0, 0.0,  0.0, 0.0, // bottom left   // 2
            top,    left,  0.0,  0.0, 0.0, 1.0,  0.0, 1.0, // top left      // 3
        ];

        let indexes_1: Vec<i32> = vec![0, 1, 3, 1, 2, 3];

        let triangle_1 = Geometry::from_data(
            &data,
            &indexes_1,
            gl_program_1,
            Vec::new(),
            8,
            &[(3, 0), (3, 3), (2, 6)],
        );

        return Ok(triangle_1);
    }

    pub fn cube(
        width: f32,
        height: f32,
        depth: f32,
        frag: &'a str,
        vert: &'a str,
    ) -> anyhow::Result<Geometry<'a>> {
        let frag_shader = Shader::from_fragment_src(frag)?;
        let vertex_shader = Shader::from_vertex_src(vert)?;

        let gl_program_1 = GLProgram::from_shaders(&[&frag_shader, &vertex_shader])?;

        let top: f32 = (height / 2.0) * 0.01;
        let bottom: f32 = -top;
        let right: f32 = (width / 2.0) * 0.01;
        let left: f32 = -right;
        let front: f32 = (depth / 2.0) * 0.01;
        let back: f32 = -front;

        #[rustfmt::skip]
        let cube: Vec<f32> = vec![
            // vertices       |// Colors     // Texture
            // Back
            -0.5, -0.5, back, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.5, -0.5, back, 0.0, 0.0, 0.0, 1.0, 0.0,
             0.5,  0.5, back, 0.0, 0.0, 0.0, 1.0, 1.0,
             0.5,  0.5, back, 0.0, 0.0, 0.0, 1.0, 1.0,
            -0.5,  0.5, back, 0.0, 0.0, 0.0, 0.0, 1.0,
            -0.5, -0.5, back, 0.0, 0.0, 0.0, 0.0, 0.0,

            // Face
            -0.5, -0.5,  front, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.5, -0.5,  front, 0.0, 0.0, 0.0, 1.0, 0.0,
             0.5,  0.5,  front, 0.0, 0.0, 0.0, 1.0, 1.0,
             0.5,  0.5,  front, 0.0, 0.0, 0.0, 1.0, 1.0,
            -0.5,  0.5,  front, 0.0, 0.0, 0.0, 0.0, 1.0,
            -0.5, -0.5,  front, 0.0, 0.0, 0.0, 0.0, 0.0,

            // Lateral Left
            left,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
            left,  0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
            left, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
            left, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
            left, -0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
            left,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,

            // Lateral Right
             right,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
             right,  0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
             right, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
             right, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
             right, -0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
             right,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,

             // Floor
            -0.5, bottom, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
             0.5, bottom, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
             0.5, bottom,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
             0.5, bottom,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
            -0.5, bottom,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
            -0.5, bottom, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,

            // Ceiling
            -0.5,  top, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
             0.5,  top, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
             0.5,  top,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
             0.5,  top,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
            -0.5,  top,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
            -0.5,  top, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0
        ];

        #[rustfmt::skip]
        let cube_indexes: Vec<i32> = vec![
             0,  1,  2,  3,  4,  5,
             6,  7,  8,  9, 10, 11,
            12, 13, 14, 15, 16, 17,
            18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29,
            30, 31, 32, 33, 34, 35,
        ];

        // TODO: set the textures dinamically, and not initially, if possible.
        let image_data = Texture::from_files(&[
            &TextureDescriptor::new("wall.jpg", "texture_1"),
            &TextureDescriptor::new("tux.png", "texture_2"),
        ])?;

        let cube_1 = Geometry::from_data(
            &cube,
            &cube_indexes,
            gl_program_1,
            image_data,
            8,
            &[(3, 0), (3, 3), (2, 6)],
        );

        return Ok(cube_1);
    }
}
