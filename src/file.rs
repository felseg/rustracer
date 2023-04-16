use std::fs::File;
use std::io::prelude::*;

use crate::vec3::Vec3;

fn create_file(filename: &str) -> File {
    match File::create(filename) {
        Ok(file) => return file,
        Err(_) => panic!("failed to create file"),
    };
}

#[allow(unused_must_use)]
pub fn write_test_image() {
    let width = 256;
    let height = 256;

    let mut file = create_file("example.ppm");

    write!(file, "P3\n{} {}\n255\n", width, height);

    for j in (0..width).rev() {
        for i in 0..height {
            let r = i as f64 / ((width - 1) as f64);
            let g = j as f64 / ((height - 1) as f64);
            let b = 0.25f64;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            write!(file, "{} {} {}\n", ir, ig, ib);
        }
    }
}

#[allow(unused)]
#[allow(unused_must_use)]
pub fn write_image(width: usize, height: usize, image: Vec<Vec<Vec3>>) {
    let mut file = create_file("out.ppm");

    write!(file, "P3\n{} {}\n255\n", width, height);

    for j in (0..width).rev() {
        for i in 0..height {
            let r = i as f64 / ((width - 1) as f64);
            let g = j as f64 / ((height - 1) as f64);
            let b = 0.25f64;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            write!(file, "{} {} {}\n", ir, ig, ib);
        }
    }
}
