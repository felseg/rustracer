use rand::Rng;

use crate::vec3::{unit_vector, Vec3};

pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degress: f64) -> f64 {
    degress * PI / 180.
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn mm_random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3(random_double(), random_double(), random_double());
        if p.length_squared() >= 1. {
            continue;
        };
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}
