use crate::{Color, HitResult, Ray, World};

const MAX_DEPTH: u32 = 10;

pub fn trace(ray: &Ray, world: &World, depth: u32) -> Color {
    let mut result = HitResult::no_hit();
    if depth < MAX_DEPTH {
        for object in &world.objects {
            let r = object.intersect(&ray);
            if r.ray_hit && r.distance < result.distance {
                result = r;
            }
        }
    } else {
        return Color::new(0.001, 0.001, 0.001);
    }

    if result.ray_hit {
        let material = result.material.expect("No material found..");
        return material.shade(ray, &result, world, depth);
    } else {
        let view_normal = ray.direction.normalize();
        let t = 0.5 * (view_normal.y + 1.0);
        return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    }
}
