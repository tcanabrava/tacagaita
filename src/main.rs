extern crate glfw;
extern crate gl;
extern crate image;
extern crate nalgebra_glm as glm;

use glfw::{Action, Context, Key};

mod textures;
mod shader;
mod helpers;
mod geometry;
mod gl_program;
mod transformation;
mod scene;

use shader::*;
use geometry::*;
use textures::*;
use scene::Scene;
use gl_program::GLProgram;

fn main() -> Result<(), anyhow::Error> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = create_window(&mut glfw);

    let triangle_frag = Shader::from_fragment_src(include_str!("shaders/main_triangle.frag"))?;
    let triangle_vert = Shader::from_vertex_src(include_str!("shaders/main_triangle.vert"))?;
    let triangle2_vert = Shader::from_vertex_src(include_str!("shaders/main_triangle.vert"))?;
    let color_blue_frag = Shader::from_fragment_src(include_str!("shaders/set_color_blue.frag"))?;
    let gl_program_1 = GLProgram::from_shaders(&[&triangle_vert, &triangle_frag])?;
 //   let gl_program_2 = GLProgram::from_shaders(&[&triangle2_vert, &color_blue_frag])?;

    let image_data = Texture::from_files(&[
        &TextureDescriptor{
            name:"/data/Projects/tocagaita/src/textures/wall.jpg",
            uniform: "texture_1",
        },
        &TextureDescriptor{
            name:"/data/Projects/tocagaita/src/textures/tux.png",
            uniform: "texture_2",
        },
    ])?;

    /*
    let triangle1: Vec<f32> = vec![
        // vertices     |// Colors      // Texture
        0.5,  0.5, 0.0,  1.0, 0.0, 0.0,  1.0, 1.0, // top right     // 0
        0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  1.0, 0.0, // bottom right  // 1
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,  0.0, 0.0, // bottom left   // 2
        -0.5,  0.5, 0.0,  0.0, 0.0, 1.0, 0.0, 1.0, // top left      // 3
    ];

    let indexes_1: Vec<i32> = vec![
        0, 1, 3,
        1, 2, 3
    ];

    let mut triangle_1 = Geometry::from_data(
        &triangle1,
        &indexes_1,
        gl_program_1,
        Vec::new(),
        8,
        &[(3,0), (3,3), (2,6)]);
    */

    let cube: Vec<f32> = vec![
        // vertices       |// Colors     // Texture
        // Bottom
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
         0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
         0.5,  0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
         0.5,  0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
        -0.5,  0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,

        // Face
        -0.5, -0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
         0.5, -0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
         0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
         0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
        -0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
        -0.5, -0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,

        // Lateral Left
        -0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
        -0.5,  0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
        -0.5, -0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
        -0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,

        // Lateral Right
         0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
         0.5,  0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
         0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
         0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
         0.5, -0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
         0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,

         // Floor
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
         0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
         0.5, -0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
         0.5, -0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
        -0.5, -0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,

        // Ceiling
        -0.5,  0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
         0.5,  0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0,
         0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
         0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 1.0, 0.0,
        -0.5,  0.5,  0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
        -0.5,  0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0
    ];

    let cube_indexes: Vec<i32> = vec![
         0,  1,  2,  3,  4,  5,
         6,  7,  8,  9, 10, 11,
        12, 13, 14, 15, 16, 17,
        18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29,
        30, 31, 32, 33, 34, 35,
    ];

    let mut cube_1 = Geometry::from_data(
        &cube,
        &cube_indexes,
        gl_program_1,
        image_data,
        8,
        &[(3,0), (3,3), (2,6)]);

    cube_1.set_positions(vec![
//        glm::vec3( 0.0,  0.0,  0.0),
//        glm::vec3( 0.2,  0.1, 0.3),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3( 2.4, -0.4, -3.5),
        glm::vec3(-1.7,  3.0, -7.5),
        glm::vec3( 1.3, -2.0, -2.5),
        glm::vec3( 1.5,  2.0, -2.5),
        glm::vec3( 1.5,  0.2, -1.5),
        glm::vec3(-1.3,  1.0, -1.5)
    ]);

//    let triangle_matrix = triangle_1.matrix_mut();
//    triangle_matrix.rotate(Angle::X(-55.0));

    let mut scene = Scene::new();
    scene.geometries().push(cube_1);
//    scene.geometries().push(triangle_2);

    scene.view().translade(0.0, 0.0, -3.0);
    scene.projection().perspactive(45.0, 800.0 / 600.0, 0.1, 100.0);

    let(width, height) = window.get_framebuffer_size();
    unsafe {
        gl::Viewport(0, 0, width, height);
        gl::ClearColor(0.1, 0.3, 0.3, 1.0);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

        scene.render();

        window.swap_buffers();
    }

    return Ok(());
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
