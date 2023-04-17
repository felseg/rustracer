use crate::{
    ray::Ray,
    vec3::{dot, Vec3},
};
pub mod sphere;

#[derive(Clone)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.dir, &outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    #[allow(unused)]
    fn hit(&mut self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut Hit) -> bool {
        false
    }
}
