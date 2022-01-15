extern crate minifb;
use glam::Vec3A;
use minifb::{Key, Window, WindowOptions};
use std::{thread, time::Instant};

pub mod raytracer;
pub use raytracer::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

const MAX_DEPTH: u32 = 3;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut accumilator: Vec<Color> = vec![Color::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
    let mut accumilator_frames: u32 = 0;

    let mut cam = Camera::new();
    let mut world: World = World::new();
    for i in 0..3 {
        for j in 0..3 {
            world.objects.push(Box::new(Sphere::new(
                Vec3A::new(i as f32 * 6.0, 2.0, j as f32 * 6.0),
                2.0,
            )));
        }
    }

    world
        .objects
        .push(Box::new(Sphere::new(Vec3A::new(5.0, -100.0, 5.0), 100.0)));

    let mut window = Window::new(
        "Rust-tracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~120 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(8300)));

    let mut last = Instant::now();
    let mut dt: f32 = 0.0;

    let mut passed_frames: u32 = 0;
    let mut passed_time: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update keys
        window.get_keys().iter().for_each(|key| match key {
            // Camera movement
            Key::W => {
                cam.set_position(cam.position + cam.view_direction * 10.0 * dt);
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::S => {
                cam.set_position(cam.position - cam.view_direction * 10.0 * dt);
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::A => {
                cam.set_position(
                    cam.position
                        + Vec3A::new(-cam.view_direction.z, 0.0, cam.view_direction.x) * 10.0 * dt,
                );
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::D => {
                cam.set_position(
                    cam.position
                        - Vec3A::new(-cam.view_direction.z, 0.0, cam.view_direction.x) * 10.0 * dt,
                );
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::Q => {
                cam.set_position(cam.position + Vec3A::new(0.0, -1.0, 0.0) * 10.0 * dt);
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::E => {
                cam.set_position(cam.position + Vec3A::new(0.0, 1.0, 0.0) * 10.0 * dt);
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            // Camera Rotation
            Key::Left => {
                cam.rotate_y(-1.0 * dt);
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::Right => {
                cam.rotate_y(1.0 * dt);
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::Up => {
                cam.rotate_x(-1.0 * dt);
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::Down => {
                cam.rotate_x(1.0 * dt);
                reset_accumilator(&mut accumilator, &mut accumilator_frames)
            }
            Key::R => reset_accumilator(&mut accumilator, &mut accumilator_frames),
            _ => (),
        });

        // Update raytracer
        accumilator_frames += 1;
        let accumilator_inverse = 1.0 / accumilator_frames as f32;
        for x in 0..WIDTH - 1 {
            for y in 0..HEIGHT - 1 {
                let uv_x: f32 = (x as f32) / (WIDTH as f32);
                let uv_y: f32 = (y as f32) / (HEIGHT as f32);
                let ray = cam.construct_ray(uv_x, uv_y);
                accumilator[y * WIDTH + x] += trace(&ray, &world, 0);
                buffer[y * WIDTH + x] = (accumilator[y * WIDTH + x] * accumilator_inverse).to_u32();
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        let now = Instant::now();
        dt = now.duration_since(last).as_secs_f32();
        last = now;

        passed_frames += 1;
        passed_time += dt;
        if passed_frames >= 100 {
            let average = passed_time / passed_frames as f32;
            println!("Average FPS over 100 frames: {}", 1.0 / average);
            println!("Accumilated frames: {}", accumilator_frames);
            passed_frames = 0;
            passed_time = 0.0;
        }
    }
}

fn trace(ray: &Ray, world: &World, depth: u32) -> Color {
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
        let reflect_ray = Ray::new(result.position + result.normal * 0.001, result.normal);
        let reflect = trace(&reflect_ray, world, depth + 1);
        let mut scatter_ray = Ray::new(result.position + result.normal * 0.001, result.normal);
        scatter_ray.randomize_direction_unit_sphere();
        let scatter = trace(&scatter_ray, world, depth + 1);

        //return scatter * 0.2 + reflect * 0.2 + Color::new(0.8, 0.0, 0.0) * 0.6;
        return (scatter * 0.8 + reflect * 0.2) * 0.5;
        //return Color::new(result.normal.x, result.normal.y, result.normal.z);
    } else {
        let view_normal = ray.direction.normalize();
        let t = 0.5 * (view_normal.y + 1.0);
        return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
    }
}

fn reset_accumilator(accumilator: &mut Vec<Color>, frame_count: &mut u32) {
    *frame_count = 0;

    for i in 0..(WIDTH - 1) * (HEIGHT - 1) {
        accumilator[i] = Color::new(0.0, 0.0, 0.0);
    }
}
