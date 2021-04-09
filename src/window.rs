extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};

use crate::scene::Scene;
pub struct WindowContainer {
    glfw: glfw::Glfw,
    window: glfw::Window,
    event_handler: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    scene: Option<Scene>
}

impl WindowContainer {
    pub fn new(title: &str) -> WindowContainer {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(
                800,
                600,
                title,
                glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.make_current();

        WindowContainer {
            glfw: glfw,
            window: window,
            event_handler: events,
            scene: None,
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = Some(scene);
    }

    pub fn event_loop(&mut self) {
        let (width, height) = self.window.get_framebuffer_size();
        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::ClearColor(0.1, 0.3, 0.3, 1.0);
        }

        while !self.window.should_close() {
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&mut self.event_handler) {
                handle_window_event(&mut self.window, event);
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            if let Some(scene) = &mut self.scene {
                scene.render();
            }

            self.window.swap_buffers();
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    // println!("Window event received {:?}", event);
    match event {
        glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
            key_event(window, key, scancode, action, modifiers)
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

fn key_event(window: &mut glfw::Window, key: Key, _scancode: i32, action: Action, _modifiers: glfw::Modifiers) {
    match (key, action) {
        (Key::Escape, Action::Press) => {
            println!("Closing Window");
            window.set_should_close(true);
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

fn resize_event(width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width, height) };
}
