use std::f32::INFINITY;

use crate::Ray;
use glam::Mat3A;
use glam::Vec3A;

pub struct Camera {
    distance: f32,
    angle: Vec3A,
    pub position: Vec3A,
    pub view_direction: Vec3A,
    screen_center: Vec3A,
    top_left_corner: Vec3A,
    top_right_corner: Vec3A,
    bottom_left_corner: Vec3A,
}

impl Camera {
    pub fn new() -> Camera {
        let distance = 1.0;
        let position = Vec3A::new(0.0, 2.0, 0.0);
        let view_direction = Vec3A::new(0.0, 0.0, 1.0);
        let screen_center = position + (view_direction * distance);

        Camera {
            distance: distance,
            angle: Vec3A::new(0.0, 0.0, 0.0),
            position: position,
            view_direction: view_direction,
            screen_center: screen_center,
            top_left_corner: screen_center + Vec3A::new(-1.0, 1.0, 0.0),
            top_right_corner: screen_center + Vec3A::new(1.0, 1.0, 0.0),
            bottom_left_corner: screen_center + Vec3A::new(-1.0, -1.0, 0.0),
        }
    }
    pub fn construct_ray(&self, x: f32, y: f32) -> Ray {
        let p_uv = self.top_left_corner
            + ((self.top_right_corner - self.top_left_corner) * x)
            + ((self.bottom_left_corner - self.top_left_corner) * y);
        let direction = (p_uv - self.position).normalize();

        Ray {
            origin: self.position,
            direction: direction,
            t: INFINITY,
        }
    }
    fn apply_rotation(&mut self) {
        let rot_x = Mat3A::from_rotation_x(self.angle.x);
        let rot_y = Mat3A::from_rotation_y(self.angle.y);

        self.view_direction = rot_y * rot_x * Vec3A::new(0.0, 0.0, 1.0);
        self.screen_center = self.position + (self.distance * self.view_direction);

        self.top_left_corner = self.screen_center + ((rot_y * rot_x) * Vec3A::new(-1.0, 1.0, 0.0));
        self.top_right_corner = self.screen_center + ((rot_y * rot_x) * Vec3A::new(1.0, 1.0, 0.0));
        self.bottom_left_corner =
            self.screen_center + ((rot_y * rot_x) * Vec3A::new(-1.0, -1.0, 0.0));
    }
    pub fn rotate_x(&mut self, angle: f32) {
        self.angle.x += angle;
        self.apply_rotation();
    }
    pub fn rotate_y(&mut self, angle: f32) {
        self.angle.y += angle;
        self.apply_rotation();
    }
    pub fn set_position(&mut self, pos: Vec3A) {
        self.position = pos;
        self.apply_rotation();
    }
}
