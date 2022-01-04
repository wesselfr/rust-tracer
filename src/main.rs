extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use glam::{Vec3A};

pub mod raytracer;
pub use raytracer::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut cam = Camera::new();
    let sphere = Sphere::new(Vec3A::new(0.0, 2.0, 6.0), 2.0);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Update keys
        window.get_keys().iter().for_each(|key|
            match key {
                // Camera movement
                Key::W => cam.set_position(cam.position + cam.view_direction),
                Key::S => cam.set_position(cam.position - cam.view_direction),
                Key::A => cam.set_position(cam.position + Vec3A::new(-cam.view_direction.z, 0.0, cam.view_direction.x)),
                Key::D => cam.set_position(cam.position - Vec3A::new(-cam.view_direction.z, 0.0, cam.view_direction.x)),
                Key::Q => cam.set_position(cam.position + Vec3A::new(0.0, -1.0, 0.0)),
                Key::E => cam.set_position(cam.position + Vec3A::new(0.0, 1.0, 0.0)),
                // Camera Rotation
                Key::Left => cam.rotate_y(-0.1),
                Key::Right => cam.rotate_y(0.1),
                Key::Up => cam.rotate_x(-0.1),
                Key::Down => cam.rotate_x(0.1),
                _ => (),
            }
        );

        // Update raytracer
        for x in 0..WIDTH-1 {
            for y in 0..HEIGHT-1 {
                let uv_x:f32 = (x as f32) / (WIDTH as f32);
                let uv_y:f32 = (y as f32) / (HEIGHT as f32);
                let mut ray = cam.construct_ray(uv_x, uv_y);

                // Background colour.
                //buffer[y * WIDTH + x] = 40;
                if sphere.intersect(&mut ray) {
                    let normal = sphere.get_normal(ray.get_point());

                    let col = Color::new(normal.x, normal.y, normal.z);
                    buffer[y * WIDTH + x] = col.to_u32();
                }
                else{
                    let view_normal = cam.view_direction.normalize();
                    let t = 0.5*(view_normal.y + 1.0);
                    let color = Color::new(1.0, 1.0, 1.0)*(1.0-t) + Color::new(0.5, 0.7, 1.0)*t;
                    buffer[y * WIDTH + x] = color.to_u32();
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
