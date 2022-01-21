use crate::{trace, Color, HitResult, Ray, World};

pub trait Material {
    fn shade(&self, ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color;
    fn get_color(&self) -> Color;
}

pub struct Lambert {
    albedo: Color,
}

impl Lambert {
    pub fn new(color: Color) -> Lambert {
        Lambert { albedo: color }
    }

    fn reflect(&self, result: &HitResult, world: &World, depth: u32) -> Color {
        let reflect_ray = Ray::new(result.position + result.normal * 0.001, result.normal);
        trace(&reflect_ray, world, depth + 1)
    }

    fn scatter(&self, result: &HitResult, world: &World, depth: u32) -> Color {
        let mut scatter_ray = Ray::new(result.position + result.normal * 0.001, result.normal);
        scatter_ray.randomize_direction_unit_sphere();
        trace(&scatter_ray, world, depth + 1)
    }
}

impl Material for Lambert {
    fn shade(&self, _ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color {
        let reflect = self.reflect(&result, world, depth);
        let scatter = self.scatter(&result, world, depth);

        return (scatter * 0.4 + reflect * 0.6) * self.get_color();
    }

    fn get_color(&self) -> Color {
        self.albedo
    }
}

pub struct ScatterOnlyMaterial {
    albedo: Color,
}

impl ScatterOnlyMaterial {
    pub fn new(color: Color) -> ScatterOnlyMaterial {
        ScatterOnlyMaterial { albedo: color }
    }

    fn scatter(&self, result: &HitResult, world: &World, depth: u32) -> Color {
        let mut scatter_ray = Ray::new(result.position + result.normal * 0.001, result.normal);
        scatter_ray.randomize_direction_unit_sphere();
        trace(&scatter_ray, world, depth + 1)
    }
}

impl Material for ScatterOnlyMaterial {
    fn shade(&self, _ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color {
        let scatter = self.scatter(&result, world, depth);

        return scatter * self.get_color();
    }

    fn get_color(&self) -> Color {
        self.albedo
    }
}

pub struct GlassMaterial {}

impl GlassMaterial {
    fn refract(&self, ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color {
        let mut scatter_ray = Ray::new(result.position + result.normal * 0.001, result.normal);
        scatter_ray.randomize_direction_unit_sphere();
        trace(&scatter_ray, world, depth + 1)

        // Vector3 a = n * (rayDirection - normal * rayDirection.Dot(normal)) / nt;
        // float power = powf(rayDirection.Dot(normal), 2.f);
        // float sqr = sqrtf(1.f - (((n * n) * 1.f - power) / (nt * nt)));
        // if (sqr >= 0) {
        //     Vector3 b = normal * sqr;
        //     Vector3 refractDir = a - b;

        //     Ray refraction;
        //     refraction.direction = refractDir.Normalized();
        //     refraction.origin = point + refractDir * 0.0001f;
        //     return Trace(&refraction,lastPrimitiveIndex, ++count);
        // }
        // return { 1,0,1 };
    }
}

impl Material for GlassMaterial {
    fn shade(&self, ray: &Ray, result: &HitResult, world: &World, depth: u32) -> Color {
        let scatter = self.refract(ray, &result, world, depth);
        // Todo: Implement Beers law
        return scatter;
    }

    fn get_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
