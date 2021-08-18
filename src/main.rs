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

    let square = Geometry::cube(
        100.0,
        100.0,
        100.0,
        include_str!("shaders/main_triangle.frag"),
        include_str!("shaders/main_triangle.vert")
    )?;

    let mut scene = Scene::new();
    scene.geometries().push(square);

    scene
        .projection()
        .perspective(45.0, 800.0 / 600.0, 0.1, 100.0);

    window.event_loop(&mut scene);
    return Ok(());
}
