extern crate minifb;

use minifb::{Key, Window, WindowOptions};

pub mod raymath;
pub use raymath::{Ray, Vector3};

pub mod raytracer;
pub use raytracer::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let test_vector: Vector3 = Vector3::new(5.0, 2.0, 1.0);
    let test_vector_two: Vector3 = Vector3::new(3.0, 2.0, 1.0);

    let mut test = test_vector + test_vector_two;
    println!("Output: {}", &test);
    test = test * 10.0;
    println!("Output: {}", &test);

    let cam = Camera::new();
    let sphere = Sphere::new(Vector3::new(0.0, 2.0, 8.0), 8.0);

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
        // for i in buffer.iter_mut() {
        //     *i = 40; // write something more funny here!
        // }

        println!("Render start");
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let ray = cam.construct_ray(x as f32, y as f32);
                if sphere.intersect(ray) {
                    buffer[x * WIDTH + y] = 40;
                }
            }
        }
        println!("Render done");

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
