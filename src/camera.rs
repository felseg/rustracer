use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }

    pub fn new() -> Camera {
        let aspect_ratio = 16. / 9.;

        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Vec3(0., 0., 0.);
        let horizontal = Vec3(viewport_width, 0., 0.);
        let vertical = Vec3(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}
