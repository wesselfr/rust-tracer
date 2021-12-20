use std::f32::INFINITY;

use crate::{Ray, Vector3};

pub struct Camera {
    distance: f32,
    angle: Vector3,
    position: Vector3,
    view_direction: Vector3,
    screen_center: Vector3,
    top_left_corner: Vector3,
    top_right_corner: Vector3,
    bottom_left_corner: Vector3,
}

impl Camera {
    pub fn new() -> Camera {
        let distance = 1.0;
        let position = Vector3::new(0.0, 2.0, 0.0);
        let view_direction = Vector3::new(0.0, 0.0, 1.0);
        let screen_center = position + (view_direction * distance);

        Camera {
            distance: distance,
            angle: Vector3::new(0.0, 0.0, 0.0),
            position: position,
            view_direction: view_direction,
            screen_center: screen_center,
            top_left_corner: screen_center + Vector3::new(-1.0, 1.0, 0.0),
            top_right_corner: screen_center + Vector3::new(1.0, 1.0, 0.0),
            bottom_left_corner: screen_center + Vector3::new(-1.0, -1.0, 0.0),
        }
    }
    pub fn construct_ray(&self, x: f32, y: f32) -> Ray {
        let p_uv = self.top_left_corner
            + ((self.top_right_corner - self.top_left_corner) * x)
            + ((self.bottom_left_corner - self.top_left_corner) * y);
        let mut direction = p_uv - self.position;
        direction.normalize();

        Ray {
            origin: self.position,
            direction: direction,
            t: INFINITY,
        }
    }
}
