extern crate nalgebra_glm as glm;

pub struct Camera {
    pos: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    speed: f32,
}

impl Camera {

    /* creates a new camera. */
    pub fn new() -> Camera {
        Camera {
            pos: glm::vec3(0.0,0.0,3.0),
            front: glm::vec3(0.0,0.0,-1.0),
            up: glm::vec3(0.0,1.0,0.0),
            speed: 0.5,
        }
    }

    pub fn view(&self) -> glm::Mat4 {
        let center = self.pos.component_mul(&self.front);
        let our_view = glm::look_at(&self.pos, &center ,&self.up);
        return our_view;
    }

    // TODO: new trait `EventHandler`.
    pub fn key_event(&mut self, key: glfw::Key, _scancode: i32, action: glfw::Action, _modifiers: glfw::Modifiers) -> bool {
        let result = match (key, action) {
            (glfw::Key::W, glfw::Action::Press) => { self.pos += self.speed * self.front; true}
            (glfw::Key::S, glfw::Action::Press) => { self.pos -= self.speed * self.front; true}
            (glfw::Key::A, glfw::Action::Press) => { self.pos -= glm::normalize(&glm::cross(&self.front, &self.up)) * self.speed; true}
            (glfw::Key::D, glfw::Action::Press) => { self.pos += glm::normalize(&glm::cross(&self.front, &self.up)) * self.speed; true}
            _ => { false }
        };

        return result;
    }
}