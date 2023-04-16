use std::{fs::File, io::Write};

use ray::Ray;
use vec3::{unit_vector, Vec3};

mod file;
mod rand;
mod ray;
mod utils;
mod vec3;

fn create_file(filename: &str) -> File {
    match File::create(filename) {
        Ok(file) => return file,
        Err(_) => panic!("failed to create file"),
    };
}

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = unit_vector(&ray.dir);
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t)
        * Vec3 {
            x: 0.,
            y: 1.,
            z: 1.,
        }
        + t * Vec3 {
            x: 0.5,
            y: 0.7,
            z: 0.3,
        }
}

#[allow(unused)]
fn write(vec: &Vec3, out: &mut File) {
    let ir = (255.999 * vec.x) as i32;
    let ig = (255.999 * vec.y) as i32;
    let ib = (255.999 * vec.z) as i32;

    write!(out, "{} {} {}\n", ir, ig, ib);
}

#[allow(unused)]
fn write_header(width: i32, height: i32, out: &mut File) {
    write!(out, "P3\n{} {}\n255\n", width, height);
}

#[allow(unused)]
fn main() {
    // Image
    // Render
    let aspect_ratio = 16. / 9.;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;

    let mut file = create_file("out.ppm");

    write_header(width, height, &mut file);

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    let vertical = Vec3 {
        x: 0.,
        y: viewport_height,
        z: 0.,
    };

    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.,
        z: 0.,
    };

    let lower_left_corner = origin
        - horizontal / 2.
        - vertical / 2.
        - Vec3 {
            x: 0.,
            y: 0.,
            z: focal_length,
        };

    for j in (0..width).rev() {
        for i in 0..height {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let r = Ray {
                origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let color = ray_color(&r);

            write(&color, &mut file)
        }
    }
}
