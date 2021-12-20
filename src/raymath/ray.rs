use crate::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            t: f32::INFINITY,
        }
    }
}
