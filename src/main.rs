use std::{
    f64::INFINITY,
    io::{stdout, Stdout, Write},
};

use image::{Rgb, RgbImage};
use rayon::prelude::*;

use crate::hittables::Hittables::*;
use crate::materials::Material::{Dielectric, Lambertian, Metal};
use camera::Camera;
use crossterm::style::Stylize;
use hittable::Hit;
use hittables::{hit, Hittables};
use materials::{color_emitted, scatter};
use ray::Ray;
use utils::{clamp, random_double};
use vec3::Vec3;

mod camera;
mod file;
mod hittable;
mod hittables;
mod materials;
mod rand;
mod ray;
mod utils;
mod vec3;

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

    if !hit(world, &ray, 0.001, INFINITY, &mut rec) {
        return Vec3(0.0, 0.0, 0.0);
    }

    let mut scattered = Ray {
        origin: Vec3(0., 0., 0.),
        dir: Vec3(0., 0., 0.),
    };

    let mut attenuation = Vec3(0., 0., 0.);
    let emitted = color_emitted(rec.material);

    if !scatter(rec.material, ray, &rec, &mut attenuation, &mut scattered) {
        return emitted;
    }

    emitted + attenuation * ray_color(&scattered, world, depth - 1)
}

fn color_rgb(vec: &Vec3, samples_per_pixel: i32) -> (f64, f64, f64) {
    let mut r = vec.0;
    let mut g = vec.1;
    let mut b = vec.2;

    let scale = 1.0 / samples_per_pixel as f64;

    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    (
        256. * clamp(r, 0., 0.999),
        256. * clamp(g, 0., 0.999),
        256. * clamp(b, 0., 0.999),
    )
}

fn main() {
    //Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 16;
    let max_depth = 8;

    let mut hittables = Vec::new();

    hittables.push(Sphere(
        Vec3(0., -100.5, -1.),
        100.,
        Metal(0.8, 0.8, 0.8, 0.2),
    ));
    hittables.push(Sphere(Vec3(0.9, 0., -1.), 0.5, Metal(1., 1., 1., 0.0)));
    hittables.push(Sphere(Vec3(-0.9, 0., -1.), -0.5, Dielectric(2.2)));
    hittables.push(Sphere(
        Vec3(0., -0.25, -0.25),
        0.25,
        Lambertian(0.94, 0.81, 0.66),
    ));

    hittables.push(Sphere(
        Vec3(1.25, -0.25, -0.25),
        0.25,
        materials::Material::Lambertian(0.6, 0.76, 0.73),
    ));
    hittables.push(Sphere(
        Vec3(-1.25, -0.25, -0.25),
        0.25,
        materials::Material::Lambertian(0.84, 0.55, 0.8),
    ));
    // hittables.push(Sphere(
    //     Vec3(-0.25, -0.25, 0.5),
    //     0.1,
    //     materials::Material::Metal(1., 1., 1., 0.3),
    // ));
    // hittables.push(Sphere(
    //     Vec3(0.25, -0.25, 0.5),
    //     0.1,
    //     materials::Material::Metal(0.71, 0.46, 0.16, 0.3),
    // ));

    // for a in -10..11 {
    //     for b in -10..11 {
    //         if a == -10 && b == -10 {
    //             continue;
    //         }
    //         let choose_mat = random_double();
    //         let center = Vec3(
    //             a as f64 + random_double() * 0.5,
    //             0.3 + random_double() * 1.5,
    //             b as f64 + random_double() * 0.5,
    //         );

    //         if choose_mat < 0.7 {
    //             hittables.push(Sphere(
    //                 center,
    //                 0.3 - random_double() * 0.1,
    //                 Lambertian(random_double(), random_double(), random_double()),
    //             ))
    //         } else if choose_mat < 0.8 {
    //             hittables.push(Sphere(
    //                 center,
    //                 0.3 - random_double() * 0.1,
    //                 Metal(0.9, 0.9, 0.9, random_double() * 0.1),
    //             ))
    //         } else if choose_mat < 0.9 {
    //             hittables.push(Sphere(center, 0.3, Dielectric(1.3)))
    //         } else {
    //             //hittables.push(Sphere(center + Vec3(0., 20., 0.), 6., Light(1., 1., 1.)));
    //         }
    //     }
    // }

    let world = HittableObjects(hittables);
    //Camera

    let camera = Camera::new(
        Vec3(0., 0.2, 2.),
        Vec3(0., 0., 0.),
        Vec3(0., 1., 0.),
        50.,
        aspect_ratio,
    );

    //Render
    let mut stdout = stdout();

    let mut pixels: Vec<Vec<Vec3>> = vec![Vec::new(); image_height as usize];

    pixels.par_iter_mut().enumerate().for_each(|x| {
        for i in 0..image_width {
            let mut pixel_color = Vec3(0., 0., 0.);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (x.0 as f64 + random_double()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, max_depth);
            }
            x.1.push(pixel_color);
        }
    });

    let mut img = RgbImage::new(image_width as u32, image_height as u32);

    for (y, x_vec) in pixels.iter().rev().enumerate() {
        for (x, pixel) in x_vec.iter().enumerate() {
            let c = color_rgb(pixel, samples_per_pixel);
            img.put_pixel(x as u32, y as u32, Rgb([c.0 as u8, c.1 as u8, c.2 as u8]))
        }
    }

    img.save("out.png").unwrap();
    finished(&mut stdout);
}

fn finished(stdout: &mut Stdout) {
    stdout
        .write_all(format!("{}", "Render saved to out.png.\n".bold().green()).as_bytes())
        .unwrap();
}

// fn progress(stdout: &mut Stdout, current: i32, image_height: i32) {
//     let percentage = current as f64 / image_height as f64;
//     let rgb_value = (percentage * 255.) as u8;

//     let current_color = Color::Rgb {
//         r: rgb_value,
//         g: 255 - rgb_value,
//         b: 0,
//     };

//     stdout.queue(cursor::SavePosition).unwrap();
//     stdout
//         .write_all(format!("{} ", "Rendering to out.ppm: ".bold().dark_magenta(),).as_bytes())
//         .unwrap();
//     execute!(
//         stdout,
//         SetForegroundColor(current_color),
//         Print(format!("{:.2}", 100. - percentage * 100.))
//     )
//     .unwrap();

//     stdout.flush().unwrap();
//     stdout.queue(cursor::RestorePosition).unwrap();
//     stdout
//         .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
//         .unwrap();
// }
