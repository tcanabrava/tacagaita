extern crate glfw;
extern crate gl;
extern crate image;

use glfw::{Action, Context, Key};
use image::io::Reader as ImageReader;

use image::{GenericImageView};

mod shader;
mod helpers;
mod geometry;

use shader::*;
use geometry::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = create_window(&mut glfw);

    let triangle_frag = Shader::from_fragment_src(
    include_str!("shaders/main_triangle.frag"))
        .expect("Error returning the fragment shader");

    let triangle_vert = Shader::from_vertex_src(
    include_str!("shaders/main_triangle.vert"))
        .expect("Error returning the vertex shader");

    let triangle2_vert = Shader::from_vertex_src(
        include_str!("shaders/main_triangle.vert"))
            .expect("Error returning the vertex shader");

    let color_blue_frag = Shader::from_fragment_src(
        include_str!("shaders/set_color_blue.frag"))
        .expect("Error loading the blue fragment shader");

    let gl_program_1 = GLProgram::from_shaders(&[&triangle_vert, &triangle_frag])
        .expect("Error creating the gl program");

    let gl_program_2 = GLProgram::from_shaders(&[&triangle2_vert, &color_blue_frag])
        .expect("Error creating the blue shader program");

    let image_data = ImageReader::open("/data/Projects/tocagaita/src/textures/wall.jpg")
        .expect("Could not load texture.")
        .decode()
        .expect("Could not decode texture");

    //-------- Texture -----------
    // Jesus, that's complex.
    let mut texture_id = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        // How the texture should behave when it ends
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);

        // Visualization Filters.
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);

        // Load the texture in memory
        gl::TexImage2D(
            gl::TEXTURE_2D,     // Type of Texture
            0,                  // Minimap Level
            gl::RGB as gl::types::GLint,            // Type of image to be stored
            image_data.width() as gl::types::GLint,
            image_data.height() as gl::types::GLint,
            0,                  // Always zero. Legacy.
            gl::RGB,            // Type of image to be read
            gl::UNSIGNED_BYTE,  // Type of values we are passing.
            image_data.as_rgb8().expect("Error converting").as_ptr() as *const std::ffi::c_void
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let triangle1: Vec<f32> = vec![
        0.5,  0.5, 0.0,  // top right
        0.5, -0.5, 0.0,  // bottom right
        -0.5, -0.5, 0.0,  // bottom left
        -0.5,  0.5, 0.0   // top left
    ];

    let triangle2: Vec<f32> = vec![
        0.1,  -0.2, 0.0,
        0.15, -0.1, 0.0,
        0.2,  -0.2, 0.0
    ];

    let indexes_1: Vec<i32> = vec![
        0, 1, 3,
        1, 2, 3
    ];

    let indexes_2: Vec<i32> = vec![
        0, 1, 2,
    ];

    let triangle_1 = Geometry::from_data(
        &triangle1,
        &indexes_1,
        gl_program_1,
        3,
        &[0]);

    let triangle_2 = Geometry::from_data(
        &triangle2,
        &indexes_2,
        gl_program_2,
        3,
        &[0]);

//    triangle_1.program_mut().set_float("h_offset", 0.5);

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

            for element in &[&triangle_1, &triangle_2] {
                element.program().activate();
                // TODO: Move this to element.draw();
                gl::BindVertexArray(element.vao());
                gl::DrawElements(gl::TRIANGLES, element.idx_size(), gl::UNSIGNED_INT, std::ptr::null());
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
