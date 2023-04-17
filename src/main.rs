use std::{f64::INFINITY, fs::File, io::Write};

use camera::Camera;
use hittable::{sphere::Sphere, Hit, Hittable};
use hittables::Hittables;
use materials::scatter;
use ray::Ray;
use utils::{clamp, random_double};
use vec3::{unit_vector, Vec3};

mod camera;
mod file;
mod hittable;
mod hittables;
mod materials;
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

fn ray_color(ray: &Ray, world: &mut dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3(0., 0., 0.);
    }

    let mut rec = Hit {
        point: Vec3(0., 0., 0.),
        normal: Vec3(0., 0., 0.),
        t: 0.,
        front_face: true,
        material: materials::Material::Init,
    };

    if world.hit(&ray, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray {
            origin: Vec3(0., 0., 0.),
            dir: Vec3(0., 0., 0.),
        };

        let mut attenuation = Vec3(0., 0., 0.);

        if scatter(rec.material, ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Vec3(0., 0., 0.);
    }

    let unit_direction = unit_vector(&ray.dir);
    let t = 0.5 * (unit_direction.1 + 1.);
    (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.0)
}

fn write_color(vec: &Vec3, out: &mut File, samples_per_pixel: i32) {
    let mut r = vec.0;
    let mut g = vec.1;
    let mut b = vec.2;

    let scale = 1.0 / samples_per_pixel as f64;

    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    if let Err(_) = write!(
        out,
        "{} {} {}\n",
        256. * clamp(r, 0., 0.999),
        256. * clamp(g, 0., 0.999),
        256. * clamp(b, 0., 0.999)
    ) {
        panic!("Failed writing output image to file");
    }
}

fn write_header(width: i32, height: i32, out: &mut File) {
    if let Err(_) = write!(out, "P3\n{} {}\n255\n", width, height) {
        panic!("Failed writing to file")
    }
}

fn main() {
    //Output
    let mut file = create_file("out.ppm");

    //Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 32;
    let max_depth = 16;

    write_header(image_width, image_height, &mut file);

    //World
    let mut world = Hittables { list: Vec::new() };
    world.list.push(Box::new(Sphere {
        center: Vec3(0., -100.5, -1.),
        radius: 100.,
        material: materials::Material::Lambertian(0.1, 0.7, 0.0),
    }));
    world.list.push(Box::new(Sphere {
        center: Vec3(0.5, 0., -1.),
        radius: 0.5,
        material: materials::Material::Metal(0.8, 0.8, 0.8, 0.0),
    }));
    world.list.push(Box::new(Sphere {
        center: Vec3(-0.5, 0., -1.),
        radius: 0.5,
        material: materials::Material::Metal(0.0, 0.5, 1.0, 0.5),
    }));
    world.list.push(Box::new(Sphere {
        center: Vec3(0., -0.4, -0.2),
        radius: 0.1,
        material: materials::Material::Lambertian(1., 0.1, 1.0),
    }));

    //Camera

    let camera = Camera::new();

    //Render

    for j in (0..image_height).rev() {
        println!("currently rendering row {}", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3(0., 0., 0.);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &mut world, max_depth);
            }

            write_color(&pixel_color, &mut file, samples_per_pixel);
        }
    }
}
