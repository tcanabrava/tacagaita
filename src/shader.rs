extern crate gl;
extern crate glfw;

use std::ffi::CString;

pub struct Shader {
    id: gl::types::GLuint,
}

use crate::helpers::*;

impl Shader {
    pub fn id(&self) -> gl::types::GLuint {
        return self.id;
    }

    pub fn from_vertex_src(source: &str) -> Result<Shader, std::ffi::NulError> {
        return from_src(source, gl::VERTEX_SHADER);
    }

    pub fn from_fragment_src(source: &str) -> Result<Shader, std::ffi::NulError> {
        return from_src(source, gl::FRAGMENT_SHADER);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn from_src(source: &str, kind: gl::types::GLuint) -> Result<Shader, std::ffi::NulError> {
    let id: u32 = unsafe { gl::CreateShader(kind) };
    let app_c_str = CString::new(source)?;

    unsafe {
        gl::ShaderSource(id, 1, &app_c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        check_compile_errors(id);
    }

    return Ok(Shader { id });
}

fn check_compile_errors(shader_id: u32) {
    let mut check_error = 0;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut check_error);
    }

    if check_error == 0 {
        println!("Compilation error");
        let mut error_length: i32 = 0;
        unsafe {
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut error_length);
        }
        let error_string = c_str_with_size(error_length as usize);

        unsafe {
            gl::GetShaderInfoLog(
                shader_id,
                error_length,
                std::ptr::null_mut(),
                error_string.as_ptr() as *mut gl::types::GLchar,
            );
        }

        println!("{:?}", error_string);
    }
}
