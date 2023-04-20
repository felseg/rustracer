use crate::{
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{cross, unit_vector, Vec3},
};

pub struct Camera {
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

    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.);
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}
