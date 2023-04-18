use std::{
    f64::INFINITY,
    fs::File,
    io::{stdout, BufWriter, Stdout, Write},
    ops::Index,
    sync::{Arc, Mutex},
    thread::Thread,
};

use crate::hittables::Hittables::*;
use crate::materials::Material::{Dielectric, Init, Lambertian, Metal};
use camera::Camera;
use crossterm::{
    cursor,
    style::{Print, SetForegroundColor},
    terminal, ExecutableCommand, QueueableCommand,
};
use crossterm::{
    execute,
    style::{Color, Stylize},
};
use hittable::{Hit, Hittable};
use hittables::{hit, Hittables};
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

fn ray_color(ray: &Ray, world: &Hittables, depth: i32) -> Vec3 {
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

    if hit(world, &ray, 0.001, INFINITY, &mut rec) {
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
    (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.1, 0.4, 1.0)
}

fn write_color(vec: &Vec3, out: &mut BufWriter<File>, samples_per_pixel: i32) {
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

fn write_header(width: i32, height: i32, out: &mut BufWriter<File>) {
    if let Err(_) = write!(out, "P3\n{} {}\n255\n", width, height) {
        panic!("Failed writing to file")
    }
}

fn main() {
    //Output
    let mut file = BufWriter::with_capacity(128_000_000, create_file("out.ppm"));

    //Image
    let aspect_ratio = 16. / 9.;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    println!("{}", image_height);
    let samples_per_pixel = 25;
    let max_depth = 10;

    write_header(image_width, image_height, &mut file);
    file.flush().unwrap();
    let mut hittables = Vec::new();

    hittables.push(Sphere(
        Vec3(0., -100.5, -1.),
        100.,
        Metal(0.8, 0.8, 0.8, 0.05),
    ));
    hittables.push(Sphere(Vec3(0.9, 0., -1.), 0.5, Metal(1., 1., 1., 0.0)));
    hittables.push(Sphere(Vec3(-0.9, 0., -1.), -0.5, Dielectric(2.2)));
    hittables.push(Sphere(
        Vec3(0., -0.25, -0.25),
        0.25,
        Lambertian(0.94, 0.81, 0.66),
    ));

    // hittables.list.push(Box::new(Sphere {
    //     center: Vec3(1.25, -0.25, -0.25),
    //     radius: 0.25,
    //     material: materials::Material::Lambertian(0.6, 0.76, 0.73),
    // }));
    // hittables.list.push(Box::new(Sphere {
    //     center: Vec3(-1.25, -0.25, -0.25),
    //     radius: 0.25,
    //     material: materials::Material::Lambertian(0.84, 0.55, 0.8),
    // }));
    // hittables.list.push(Box::new(Sphere {
    //     center: Vec3(-0.25, -0.25, 0.5),
    //     radius: 0.1,
    //     material: materials::Material::Metal(1., 1., 1., 0.3),
    // }));
    // hittables.list.push(Box::new(Sphere {
    //     center: Vec3(0.25, -0.25, 0.5),
    //     radius: 0.1,
    //     material: materials::Material::Metal(0.71, 0.46, 0.16, 0.3),
    // }));
    let world = HittableObjects(hittables);
    //Camera

    let camera = Camera::new(90., aspect_ratio);

    //Render
    let mut stdout = stdout();

    let pixels: Arc<Mutex<Vec<Vec<Vec3>>>> =
        Arc::new(Mutex::new(vec![Vec::new(); image_height as usize]));

    stdout.execute(cursor::Hide).unwrap();

    let cores = std::thread::available_parallelism().unwrap().get();
    unsafe {
        std::thread::scope(|s| {
            for core in 0..cores {
                s.spawn(|| {
                    for j in (core as i32..image_height).step_by(cores).rev() {
                        let mut row = Vec::with_capacity(image_width as usize);
                        for i in 0..image_width {
                            let mut pixel_color = Vec3(0., 0., 0.);

                            for _ in 0..samples_per_pixel {
                                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                                let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                                let r = camera.get_ray(u, v);

                                pixel_color += ray_color(&r, &world, max_depth);
                            }
                            row.push(pixel_color);
                        }
                        pixels.lock().unwrap()[j as usize] = row;
                    }
                });
            }
        });
    }

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            write_color(
                &pixels.lock().unwrap()[j as usize][i as usize],
                &mut file,
                samples_per_pixel,
            );
        }
    }

    file.flush().unwrap();
    finished(&mut stdout);
    stdout.execute(cursor::Show).unwrap();
}

fn finished(stdout: &mut Stdout) {
    stdout
        .write_all(format!("{}", "Render saved to out.ppm.".bold().green()).as_bytes())
        .unwrap();
}

fn progress(stdout: &mut Stdout, current: i32, image_height: i32) {
    let percentage = current as f64 / image_height as f64;
    let rgb_value = (percentage * 255.) as u8;

    let current_color = Color::Rgb {
        r: rgb_value,
        g: 255 - rgb_value,
        b: 0,
    };

    stdout.queue(cursor::SavePosition).unwrap();
    stdout
        .write_all(format!("{} ", "Rendering to out.ppm: ".bold().dark_magenta(),).as_bytes())
        .unwrap();
    execute!(
        stdout,
        SetForegroundColor(current_color),
        Print(format!("{:.2}", 100. - percentage * 100.))
    )
    .unwrap();

    stdout.flush().unwrap();
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
        .unwrap();
}
