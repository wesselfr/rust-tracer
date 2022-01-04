use glam::Vec3A;

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
}
