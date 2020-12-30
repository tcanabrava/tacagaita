extern crate glfw;
extern crate gl;
use std::ffi::{c_void};

use glfw::{Action, Context, Key};

mod shader;
use crate::shader::*;

mod helpers;
use crate::helpers::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = create_window(&mut glfw);

    let fragment_shader = Shader::from_fragment_src(
        include_str!("shaders/triangle_fragment_shader.glsl"))
            .expect("Error returning the fragment shader");

    let vertex_shader = Shader::from_vertex_src(
        include_str!("shaders/triangle_vertex_shader.glsl"))
            .expect("Error returning the vertex shader");

    let shader_program_id : u32 = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(shader_program_id, vertex_shader.id);
        gl::AttachShader(shader_program_id, fragment_shader.id);
        gl::LinkProgram(shader_program_id);
        check_link_errors(shader_program_id);
        gl::DetachShader(shader_program_id, vertex_shader.id);
        gl::DetachShader(shader_program_id, fragment_shader.id);
    }

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
          0.0, 0.5, 0.0];

    // Creates a vbo and binds the data to an array_buffer.
    // VBOs are a way to upload data to the video card
    // and that speeds up a lot of the processing time.
    let mut vbo: gl::types::GLuint = 0;

    // Creates a vao and let it store the "cache" for the
    // vbo, aparently I'll need to have one of those for each vbo.
    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1,  &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,                                      // Index of the array.
            3,                                      // number of points to consider inside of the array.
            gl::FLOAT,                              // type of the data
            gl::FALSE,                              // Dados tem que ser normalizados? (entre -1.0f e 1.0f)
            3 * std::mem::size_of::<f32>() as i32,  // size of each "block" of data
            0 as *const c_void                      // where the data begins, inside of the array
        );
    };

    let(width, height) = window.get_framebuffer_size();
    unsafe {
        gl::Viewport(0, 0, width, height);
        gl::ClearColor(0.8, 0.3, 0.3, 1.0);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program_id);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
    }
}

fn create_window(glfw: &mut glfw::Glfw) -> (glfw::Window, std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>){
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events)
        = glfw.create_window(
            800,
            600,
            "Hello this is window",
            glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    window.set_title("Hello this is window");
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    return (window, events);
}

fn check_link_errors(program_id: u32) {
    let mut check_error = 0;
    unsafe {gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut check_error); }

    if check_error == 0 {
        println!("link errors");
        let mut error_length: i32 = 0;
        unsafe { gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut error_length); }
        let error_string = c_str_with_size(error_length as usize);

        unsafe {
            gl::GetShaderInfoLog(program_id, error_length, std::ptr::null_mut(),
                error_string.as_ptr() as *mut gl::types::GLchar);
        }

        println!("{:?}", error_string);
    }
}

trait WindowEventHandler {
    fn key_event(self: &mut Self, key: Key, scancode: i32, action: Action, modifiers: glfw::Modifiers);
    fn resize_event(self: &mut Self, width: i32, height: i32);
}

impl WindowEventHandler for glfw::Window{
    fn key_event(self: &mut Self, key: Key, _scancode: i32, action: Action, _modifiers: glfw::Modifiers) {
        match (key, action) {
            (Key::Escape, Action::Press) => {
                println!("Closing Window");
                self.set_should_close(true);
            }
            _ => {}
        }
    }

    fn resize_event(self: &mut Self, width: i32, height: i32) {
        unsafe { gl::Viewport(0, 0, width, height) };
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    // println!("Window event received {:?}", event);
    match event {
        glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
            window.key_event(key, scancode, action, modifiers)
        }
        glfw::WindowEvent::Close => {
            println!("Close requested!");
        }
        glfw::WindowEvent::Refresh => {
            println!("Refresh requested");
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe { gl::Viewport(0, 0, width, height) };
        }
        _ => {}
    }
}
