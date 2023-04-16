use std::{fs::File, io::Write};

use ray::Ray;
use vec3::{dot, unit_vector, Vec3};

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
    let t = hit_sphere(
        &Vec3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        0.5,
        ray,
    );
    if t > 0. {
        let n = unit_vector(
            &(ray.at(t)
                - Vec3 {
                    x: 0.,
                    y: 0.,
                    z: -1.,
                }),
        );
        return 0.5
            * Vec3 {
                x: n.x + 1.,
                y: n.y + 1.,
                z: n.z + 1.,
            };
    }
    let unit_direction = unit_vector(&ray.dir);
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t)
        * Vec3 {
            x: 1.,
            y: 0.,
            z: 1.,
        }
        + t * Vec3 {
            x: 1.,
            y: 0.7,
            z: 0.3,
        }
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = dot(&ray.dir, &ray.dir);
    let b = 2.0 * dot(&oc, &ray.dir);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        -1.
    } else {
        (-b - f64::sqrt(discriminant)) / (2.0 * a)
    }
}

fn write(vec: &Vec3, out: &mut File) {
    let r = (255.999 * vec.x) as i32;
    let g = (255.999 * vec.y) as i32;
    let b = (255.999 * vec.z) as i32;

    if let Err(_) = write!(out, "{} {} {}\n", r, g, b) {
        panic!("Failed writing output image to file");
    }
}

fn write_header(width: i32, height: i32, out: &mut File) {
    if let Err(_) = write!(out, "P3\n{} {}\n255\n", width, height) {
        panic!("Failed writing to file")
    }
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let mut file = create_file("out.ppm");

    write_header(image_width, image_height, &mut file);

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.,
        z: 0.,
    };

    let vertical = Vec3 {
        x: 0.,
        y: viewport_height,
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

    //Render

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let dir = lower_left_corner + u * horizontal + v * vertical;

            let r = Ray {
                origin,
                dir: lower_left_corner + u * horizontal + v * vertical,
            };

            let color = ray_color(&r);

            write(&color, &mut file)
        }
    }
}
