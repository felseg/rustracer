use crate::{
    materials::Material,
    vec3::{dot, Vec3},
};

use super::Hittable;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&mut self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut super::Hit) -> bool {
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

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = self.material;

        return true;
    }
}
