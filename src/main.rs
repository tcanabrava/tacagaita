extern crate gl;
extern crate glfw;
extern crate image;
extern crate nalgebra_glm as glm;


mod geometry;
mod gl_program;
mod helpers;
mod scene;
mod shader;
mod textures;
mod transformation;
mod window;
mod camera;

use transformation::*;
use geometry::*;
use gl_program::GLProgram;
use scene::Scene;
use shader::*;
use textures::*;

fn main() -> Result<(), anyhow::Error> {
    let mut window = window::Window::new();

    let triangle2_vert = Shader::from_vertex_src(include_str!("shaders/main_triangle.vert"))?;
    let color_blue_frag = Shader::from_fragment_src(include_str!("shaders/set_color_blue.frag"))?;
    //   let gl_program_2 = GLProgram::from_shaders(&[&triangle2_vert, &color_blue_frag])?;

    let image_data = Texture::from_files(&[
        &TextureDescriptor::new("wall.jpg", "texture_1"),
        &TextureDescriptor::new("tux.png", "texture_2"),
    ])?;

    /*
    #[rustfmt::skip]
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

    #[rustfmt::skip]
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
        &[(3, 0), (3, 3), (2, 6)],
    );

    cube_1.set_positions(vec![
        //        glm::vec3( 0.0,  0.0,  0.0),
        //        glm::vec3( 0.2,  0.1, 0.3),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ]);

    cube_1.set_render_func(|transformation: &mut Transformation| {
        transformation.rotate(transformation::Angle::X(0.1));
        transformation.rotate(transformation::Angle::Y(1.0));
    });

    */

    let square = Geometry::rectangle(
        100.0,
        100.0,
        include_str!("shaders/main_triangle.frag"),
        include_str!("shaders/main_triangle.vert")
    )?;


    let mut scene = Scene::new();
    scene.geometries().push(square);
    //    scene.geometries().push(triangle_2);

    scene
        .projection()
        .perspective(45.0, 800.0 / 600.0, 0.1, 100.0);

    window.event_loop(&mut scene);
    return Ok(());
}
