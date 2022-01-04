use crate::Ray;
use crate::World;
use glam::Vec3A;

use super::world::HitResult;
use super::world::RayIntersection;

pub struct Sphere {
    pub position: Vec3A,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: Vec3A, radius: f32) -> Sphere {
        Sphere { position, radius }
    }
    pub fn get_normal(&self, point: Vec3A) -> Vec3A {
        (point - self.position) / self.radius
    }
}

impl RayIntersection for Sphere {
    fn intersect(&self, ray: &Ray) -> HitResult {
        let mut result: HitResult = HitResult::no_hit();
        let w = self.position - ray.origin;
        let proj = w.dot(ray.direction);
        let q = w - ray.direction * proj;
        let psq = q.dot(q);
        let rsq = self.radius * self.radius;

        if psq > rsq || proj < 0.0 {
            // Early return, ray did not hit.
            return result;
        }

        let rmprt = (rsq - psq).sqrt();
        let t0 = proj - rmprt;
        let t1 = proj + rmprt;

        // Ray outside sphere test.
        if t0 > 0.0 && t0 < ray.t {
            result.ray_hit = true;
            result.distance = t0;
            result.position = ray.origin + ray.direction * t0;
            result.normal = self.get_normal(result.position);
            return result;
        }
        // Ray inside sphere test.
        if t1 > 0.0 && t1 < ray.t {
            result.ray_hit = true;
            result.distance = t1;
            result.position = ray.origin + ray.direction * t1;
            result.normal = self.get_normal(result.position);
            return result;
        }

        // Ray did not hit, return no_hit result.
        result
    }
}
