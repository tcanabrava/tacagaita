extern crate glfw;
extern crate gl;
use std::ffi::{CString, CStr, c_void};

// include the OpenGL type aliases
use gl::types::*;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));

    let (mut window, events) 
        = glfw.create_window(
            300,
            300,
            "Hello this is window",
            glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    window.set_title("Hello this is window");
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();


    let(width, height) = window.get_framebuffer_size();
    unsafe { gl::Viewport(0, 0, width, height) };

    let triangulo : Vec<f32> = vec![
        -0.5, 0.5, 0.0,
        0.5, 0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

    let shader_app = include_str!("shaders/triangle.glsl");
    let shader_app_c_str = CString::new(shader_app)
        .expect("Error transforming.");

    unsafe {
        println!("Setting up shader source");
        gl::ShaderSource(vertex_shader, 1, &shader_app_c_str.as_ptr(), std::ptr::null());

        println!("Compilling shader");
        gl::CompileShader(vertex_shader);

        // Check shader compilation error.
        println!("Chekcing for shader errors");
        let mut check_error = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut check_error);
        if check_error == 0 {
            println!("Shader compilation error");
            let mut error_length: i32 = 0;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut error_length);
            let error_string = c_str_with_size(error_length as usize);

            gl::GetShaderInfoLog(vertex_shader, error_length, std::ptr::null_mut(), error_string.as_ptr() as *mut gl::types::GLchar);
            println!("{:?}", error_string);
        } else {
            println!("No shader errors reported");
        }

        gl::GenBuffers(1,  &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (triangulo.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            triangulo.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
    };

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

    }
}

fn c_str_with_size(size :usize) -> CString {
    let mut error_string = Vec::with_capacity(size as usize + 1);
    error_string.extend([b' '].iter().cycle().take(size as usize));
    return unsafe { CString::from_vec_unchecked(error_string) }
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
