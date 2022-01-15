use glam::Vec3A;
use rand::{self, Rng};

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            t: f32::INFINITY,
        }
    }
    pub fn get_point(self) -> Vec3A {
        self.origin + self.direction * self.t
    }
    pub fn randomize_direction_unit_sphere(&mut self) {
        let rand_point = self.origin
            + self.direction
            + Vec3A::new(
                rand::thread_rng().gen_range(-1.0..=1.0),
                rand::thread_rng().gen_range(-1.0..=1.0),
                rand::thread_rng().gen_range(-1.0..=1.0),
            )
            .normalize();
        self.direction = (rand_point - self.origin).normalize();
    }
}
