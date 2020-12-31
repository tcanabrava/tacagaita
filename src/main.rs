extern crate glfw;
extern crate gl;

use std::ffi::{CString};

use glfw::{Action, Context, Key};

mod shader;
mod helpers;
mod geometry;

use shader::*;
use geometry::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = create_window(&mut glfw);

    let fragment_shader = Shader::from_fragment_src(
    include_str!("shaders/triangle_fragment_shader.glsl"))
        .expect("Error returning the fragment shader");

    let vertex_shader = Shader::from_vertex_src(
    include_str!("shaders/triangle_vertex_shader.glsl"))
        .expect("Error returning the vertex shader");

    let color_blue_shader = Shader::from_fragment_src(
        include_str!("shaders/color_blue.frag.glsl"))
        .expect("Error loading the blue fragment shader");

    let gl_program_1 = GLProgram::from_shaders(&[&fragment_shader, &vertex_shader])
        .expect("Error creating the gl program");

    let gl_program_2 = GLProgram::from_shaders(&[&color_blue_shader, &vertex_shader])
        .expect("Error creating the blue shader program");

    let triangle1: Vec<f32> = vec![
        // color              |// vertice
        1.0, 0.0, 0.0, -0.1,  -0.2, 0.0,  // 0
        0.0, 1.0, 0.0, -0.15, -0.1, 0.0,  // 1
        0.0, 0.0, 1.0, -0.2,  -0.2, 0.0,  // 2
    ];

    let triangle2: Vec<f32> = vec![
        0.1,  -0.2, 0.0,
        0.15, -0.1, 0.0,
        0.2,  -0.2, 0.0
    ];

    let indexes_1: Vec<i32> = vec![
        0, 1, 2,
    ];

    let triangle_1 = Geometry::from_data(
        &triangle1,
        &indexes_1,
        gl_program_1,
        6,
        &[3, 0]);

    let triangle_2 = Geometry::from_data(
        &triangle2,
        &indexes_1,
        gl_program_2,
        3,
        &[0]);

    let(width, height) = window.get_framebuffer_size();
    unsafe {
        gl::Viewport(0, 0, width, height);
        gl::ClearColor(0.8, 0.3, 0.3, 1.0);
    }

    triangle_1.program().print_uniforms();
    triangle_2.program().print_uniforms();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            for element in &geometries {
                gl::UseProgram(element.program().id());
                gl::BindVertexArray(element.vao());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                gl::BindVertexArray(0);
            }
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
            (Key::W, Action::Press) => {
                unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
            }
            (Key::S, Action::Press) => {
                unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }
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
