use crate::{ray::Ray, utils::degrees_to_radians, vec3::Vec3};

pub struct Camera {
    vfov: f64,
    aspect_ratio: f64,
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }

    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.);
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Vec3(0., 0., 0.95);
        let horizontal = Vec3(viewport_width, 0., 0.);
        let vertical = Vec3(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

        Camera {
            vfov,
            aspect_ratio,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}
