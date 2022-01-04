use crate::Ray;
use glam::Vec3A;

pub struct Sphere {
    pub position: Vec3A,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: Vec3A, radius: f32) -> Sphere {
        Sphere { position, radius }
    }
    pub fn intersect(&self, ray: &mut Ray) -> bool {
        let w = self.position - ray.origin;
        let proj = w.dot(ray.direction);
        let q = w - ray.direction * proj;
        let psq = q.dot(q);
        let rsq = self.radius * self.radius;

        if psq > rsq || proj < 0.0 {
            return false;
        }

        let rmprt = (rsq - psq).sqrt();
        let t0 = proj - rmprt;
        let t1 = proj + rmprt;

        // Ray outside sphere test.
        if t0 > 0.0 && t0 < ray.t {
            ray.t = t0;
            return true;
        }
        // Ray inside sphere test.
        if t1 > 0.0 && t1 < ray.t {
            ray.t = t1;
            return true;
        }

        false
    }
    pub fn get_normal(&self,point: Vec3A) -> Vec3A{
        (point - self.position) / self.radius
    }
}
