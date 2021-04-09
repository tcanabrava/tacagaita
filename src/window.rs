
use crate::scene::Scene;

use std::sync::mpsc::Receiver;
use glfw::{Action, Context, Key};

pub fn event_loop(glfw: &mut glfw::Glfw, window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, scene: &mut Scene) {
    let (width, height) = window.get_framebuffer_size();
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Viewport(0, 0, width, height);
        gl::ClearColor(0.1, 0.3, 0.3, 1.0);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(window, event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        scene.render();
        window.swap_buffers();
    }
}

pub fn create_window(
    glfw: &mut glfw::Glfw,
) -> (
    glfw::Window,
    std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
) {
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    window.set_title("Hello this is window");
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    return (window, events);
}

trait WindowEventHandler {
    fn key_event(&mut self, key: Key, scancode: i32, action: Action, modifiers: glfw::Modifiers);
    fn resize_event(&mut self, width: i32, height: i32);
}

impl WindowEventHandler for glfw::Window {
    fn key_event(&mut self, key: Key, _scancode: i32, action: Action, _modifiers: glfw::Modifiers) {
        match (key, action) {
            (Key::Escape, Action::Press) => {
                println!("Closing Window");
                self.set_should_close(true);
            }
            (Key::W, Action::Press) => unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            },
            (Key::S, Action::Press) => unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            },
            _ => {}
        }
    }

    fn resize_event(&mut self, width: i32, height: i32) {
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
