use crate::vec3::{dot, Vec3};

use super::Hittable;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&mut self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, hit: &mut super::Hit) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = dot(&oc, &ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        //Nearest Root

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit.t = root;
        hit.point = ray.at(hit.t);
        let outward_normal = (hit.point - self.center) / self.radius;
        hit.set_face_normal(ray, &outward_normal);

        return true;
    }
}
