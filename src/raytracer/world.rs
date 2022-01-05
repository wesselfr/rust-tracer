use std::f32::INFINITY;

use crate::Ray;
use glam::Vec3A;

pub struct HitResult {
    pub ray_hit: bool,
    pub position: Vec3A,
    pub normal: Vec3A,
    pub distance: f32,
}

impl HitResult {
    pub fn no_hit() -> HitResult {
        HitResult {
            ray_hit: false,
            position: Vec3A::ZERO,
            normal: Vec3A::ZERO,
            distance: INFINITY,
        }
    }
}

pub trait RayIntersection {
    fn intersect(&self, ray: &Ray) -> HitResult;
}

pub struct World {
    pub objects: Vec<Box<dyn RayIntersection>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, object: Box<dyn RayIntersection>) {
        self.objects.push(object);
    }
}
