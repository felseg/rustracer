use crate::{
    hittable::Hit,
    ray::Ray,
    utils::{random_in_unit_sphere, random_unit_vector},
    vec3::{dot, reflect, unit_vector, Vec3},
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(f64, f64, f64),
    Metal(f64, f64, f64, f64),
    Init,
}

pub fn scatter(
    material: Material,
    ray: &Ray,
    rec: &Hit,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        Material::Lambertian(r, g, b) => {
            let mut scatter_direction = rec.normal + random_unit_vector();

            if scatter_direction.close_to_zero() {
                scatter_direction = rec.normal
            }

            *scattered = Ray {
                origin: rec.point,
                dir: scatter_direction,
            };
            *attenuation = Vec3(r, g, b);
            true
        }
        Material::Metal(r, g, b, fuzzy) => {
            let reflected = reflect(&unit_vector(&ray.dir), &rec.normal);
            *scattered = Ray {
                origin: rec.point,
                dir: reflected + fuzzy * random_in_unit_sphere(),
            };

            *attenuation = Vec3(r, g, b);
            dot(&scattered.dir, &rec.normal) > 0.
        }
        Material::Init => false,
    }
}
