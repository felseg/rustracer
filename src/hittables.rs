use crate::{
    hittable::{Hit, Hittable},
    materials,
    vec3::Vec3,
};

pub struct Hittables {
    pub list: Vec<Box<dyn Hittable>>,
}

#[allow(unused)]
impl Hittable for Hittables {
    fn hit(
        &mut self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        hit: &mut crate::hittable::Hit,
    ) -> bool {
        let mut temp_rec = Hit {
            point: Vec3(0., 0., 0.),
            normal: Vec3(0., 0., 0.),
            t: 0.,
            front_face: true,
            material: materials::Material::Lambertian(0., 0., 0.),
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for item in &mut self.list {
            if item.hit(&ray, t_min, t_max, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}
