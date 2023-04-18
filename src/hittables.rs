use std::sync::Arc;

use crate::{
    hittable::Hit,
    materials::{self, Material},
    vec3::{dot, Vec3},
};

#[derive(Clone)]
pub enum Hittables {
    HittableObjects(Vec<Hittables>),
    Sphere(Vec3, f64, Material),
}

pub fn hit(
    hittable_object: &Hittables,
    ray: &crate::ray::Ray,
    t_min: f64,
    t_max: f64,
    rec: &mut crate::hittable::Hit,
) -> bool {
    match hittable_object {
        Hittables::HittableObjects(list) => {
            let mut temp_rec = Hit {
                point: Vec3(0., 0., 0.),
                normal: Vec3(0., 0., 0.),
                t: 0.,
                front_face: true,
                material: materials::Material::Init,
            };
            let mut hit_anything = false;
            let mut closest_so_far = t_max;

            for item in list {
                if hit(item, &ray, t_min, t_max, &mut temp_rec) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    *rec = temp_rec.clone();
                }
            }

            hit_anything
        }
        Hittables::Sphere(center, radius, material) => {
            let oc = ray.origin - *center;
            let a = ray.dir.length_squared();
            let half_b = dot(&oc, &ray.dir);
            let c = oc.length_squared() - radius * radius;

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
            let outward_normal = (rec.point - *center) / *radius;
            rec.set_face_normal(ray, &outward_normal);
            rec.material = *material;

            true
        }
    }
}
