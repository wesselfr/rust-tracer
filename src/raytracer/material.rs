use std::f32::consts::PI;

use crate::{trace, Color, HitResult, Ray, World};

const INVPI: f32 = 1.0 / 3.14159;

pub trait Material {
    fn reflect(&self, ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color;
    fn scatter(&self, ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color;
    fn get_color(&self) -> Color;
}

pub struct Lambert {
    albedo: Color,
}

impl Lambert {
    pub fn new(r: f32, g: f32, b: f32) -> Lambert {
        Lambert {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Material for Lambert {
    fn reflect(&self, ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color {
        let reflect_ray = Ray::new(result.position + result.normal * 0.001, result.normal);
        trace(&reflect_ray, world, depth + 1)
    }

    fn scatter(&self, ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color {
        let mut scatter_ray = Ray::new(result.position + result.normal * 0.001, result.normal);
        scatter_ray.randomize_direction_unit_sphere();
        trace(&scatter_ray, world, depth + 1)
    }

    fn get_color(&self) -> Color {
        self.albedo
    }
}
