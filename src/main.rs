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

use transformation::*;
use geometry::*;
use gl_program::GLProgram;
use scene::Scene;
use shader::*;
use textures::*;

fn main() -> Result<(), anyhow::Error> {
    let mut window = window::Window::new();

    let triangle_frag = Shader::from_fragment_src(include_str!("shaders/main_triangle.frag"))?;
    let triangle_vert = Shader::from_vertex_src(include_str!("shaders/main_triangle.vert"))?;
    let triangle2_vert = Shader::from_vertex_src(include_str!("shaders/main_triangle.vert"))?;
    let color_blue_frag = Shader::from_fragment_src(include_str!("shaders/set_color_blue.frag"))?;
    let gl_program_1 = GLProgram::from_shaders(&[&triangle_vert, &triangle_frag])?;
    //   let gl_program_2 = GLProgram::from_shaders(&[&triangle2_vert, &color_blue_frag])?;

    let image_data = Texture::from_files(&[
        &TextureDescriptor::new("wall.jpg", "texture_1"),
        &TextureDescriptor::new("tux.png", "texture_2"),
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

    let cube_indexes: Vec<i32> = (0..=35).collect();

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

    //    let triangle_matrix = triangle_1.matrix_mut();
    //    triangle_matrix.rotate(Angle::X(-55.0));

    cube_1.set_render_func(|transformation: &mut Transformation| {
        transformation.rotate(transformation::Angle::X(0.1));
        transformation.rotate(transformation::Angle::Y(1.0));
    });

    let mut scene = Scene::new();
    scene.geometries().push(cube_1);
    //    scene.geometries().push(triangle_2);

    scene.view().translade(0.0, 0.0, -3.0);
    scene
        .projection()
        .perspective(45.0, 800.0 / 600.0, 0.1, 100.0);

    window.event_loop(&mut scene);
    return Ok(());
}
