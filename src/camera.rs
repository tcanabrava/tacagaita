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

    /* handles keypress. currently it's not handling anything. */
    pub fn handleKeypress(&mut self, key_press: i32) {
        match key_press {
            1 => self.pos += self.speed * self.front,
            2 => self.pos -= self.speed * self.front,
            3 => self.pos -= glm::normalize(&glm::cross(&self.front, &self.up)) * self.speed,
            4 => self.pos += glm::normalize(&glm::cross(&self.front, &self.up)) * self.speed,
            _ => println!("Oi Mundo")
        }
    }

    pub fn view(&self) -> glm::Mat4 {
        let center = self.pos.component_mul(&self.front);
        let our_view = glm::look_at(&self.pos, &center ,&self.up);

        println!("Center: {:#?} ", center);
        println!("view: {:#?} ", our_view);
        return our_view;
    }
}