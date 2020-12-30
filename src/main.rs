extern crate glfw;
extern crate gl;
use std::ffi::{CString, c_void};

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

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

    let vertex_shader_id: u32 = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let shader_app :&str = include_str!("shaders/triangle_vertex_shader.glsl");
    let shader_app_c_str = CString::new(shader_app)
        .expect("Error transforming.");

    let fragment_shader_id: u32 = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    let fragment_shader_app: &str = include_str!("shaders/triangle_fragment_shader.glsl");
    let fragment_shader_app_c_str = CString::new(fragment_shader_app)
        .expect("Error transforming");

    let shader_program_id : u32 = unsafe { gl::CreateProgram() };

    // Compile and links the program.
    // TODO: Transform this to a function
    unsafe {
        gl::ShaderSource(vertex_shader_id, 1, &shader_app_c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader_id);
        check_compile_errors(vertex_shader_id);

        gl::ShaderSource(fragment_shader_id, 1, &fragment_shader_app_c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader_id);
        check_compile_errors(fragment_shader_id);

        gl::AttachShader(shader_program_id, vertex_shader_id);
        gl::AttachShader(shader_program_id, fragment_shader_id);
        gl::LinkProgram(shader_program_id);
        check_link_errors(shader_program_id);
    }

    // Creates a vbo and binds the data to an array_buffer.
    // VBOs are a way to upload data to the video card 
    // and that speeds up a lot of the processing time.
    let mut vbo: gl::types::GLuint = 0;

    unsafe {
        gl::GenBuffers(1,  &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (triangulo.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            triangulo.as_ptr() as *const gl::types::GLvoid,
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

        gl::UseProgram(shader_program_id);

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
    unsafe {
        gl::DeleteShader(fragment_shader_id);
        gl::DeleteShader(vertex_shader_id);
    }
}

fn c_str_with_size(size :usize) -> CString {
    let mut buffer = Vec::with_capacity(size as usize + 1);
    buffer.extend([b' '].iter().cycle().take(size as usize));
    return unsafe { CString::from_vec_unchecked(buffer) }
}

fn check_compile_errors(shader_id: u32) {
    let mut check_error = 0;
    unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut check_error); }

    if check_error == 0 {
        println!("Compilation error");
        let mut error_length: i32 = 0;
        unsafe { gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut error_length); }
        let error_string = c_str_with_size(error_length as usize);

        unsafe {
            gl::GetShaderInfoLog(shader_id, error_length, std::ptr::null_mut(),
                error_string.as_ptr() as *mut gl::types::GLchar);
        }

        println!("{:?}", error_string);
    }
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
