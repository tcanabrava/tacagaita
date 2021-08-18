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
            pos: glm::vec3(0.0, 0.0, 3.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            speed: 0.5,
        }
    }

    pub fn view(&self) -> glm::Mat4 {
        let center = self.pos.component_mul(&self.front);
        let our_view = glm::look_at(&self.pos, &center, &self.up);
        return our_view;
    }

    // TODO: new trait `EventHandler`.
    pub fn key_event(
        &mut self,
        key: glfw::Key,
        _scancode: i32,
        action: glfw::Action,
        _modifiers: glfw::Modifiers,
    ) -> bool {
        use glfw::Action::Press;
        use glfw::Key;

        let result = match (key, action) {
            (Key::W, Press) => {
                self.pos += self.speed * self.front;
                true
            }
            (Key::S, Press) => {
                self.pos -= self.speed * self.front;
                true
            }
            (Key::A, Press) => {
                self.pos -= glm::normalize(&glm::cross(&self.front, &self.up)) * self.speed;
                true
            }
            (Key::D, Press) => {
                self.pos += glm::normalize(&glm::cross(&self.front, &self.up)) * self.speed;
                true
            }
            _ => false,
        };

        return result;
    }
}
