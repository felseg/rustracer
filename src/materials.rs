use crate::{
    hittable::Hit,
    ray::Ray,
    utils::{random_double, random_in_unit_sphere, random_unit_vector},
    vec3::{dot, reflect, unit_vector, Vec3},
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(f64, f64, f64),
    Metal(f64, f64, f64, f64),
    Dielectric(f64),
    Light(f64, f64, f64),
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
        Material::Dielectric(ir) => {
            *attenuation = Vec3(1., 1., 1.);
            let refraction_ratio = if rec.front_face { 1. / ir } else { ir };

            let unit_direction = unit_vector(&ray.dir);

            let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
            let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);

            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let direction;

            if cannot_refract || refractance(cos_theta, refraction_ratio) > random_double() {
                direction = reflect(&unit_direction, &rec.normal);
            } else {
                direction = refract(&unit_direction, &rec.normal, refraction_ratio);
            }

            *scattered = Ray {
                origin: rec.point,
                dir: direction,
            };
            true
        }
        Material::Light(_, _, _) => false,
        Material::Init => false,
    }
}

pub fn color_emitted(material: Material) -> Vec3 {
    match material {
        Material::Light(r, g, b) => Vec3(r, g, b),
        _ => Vec3(0., 0., 0.),
    }
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&-*uv, &n), 1.);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * *n;
    r_out_perp + r_out_parallel
}

fn refractance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= r0;
    r0 + (1. - r0) * f64::powf(1. - cosine, 5.)
}
