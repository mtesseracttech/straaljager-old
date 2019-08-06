use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use straal::{Vec3, Vec3n};

pub mod camera;
pub mod hittable;
pub mod ppm_file;
pub mod ray;
pub mod sphere;
pub mod vector_utils;

use camera::*;
use hittable::*;
use ppm_file::*;
use rand::Rng;
use ray::*;
use sphere::*;
use vector_utils::*;

pub fn get_ray_color(r: &RayN, scene: &HittableScene<f32>) -> Vec3n {
    let mut rec = HitRecord::<f32>::default();
    if scene.hit(r, 0.0, std::f32::MAX, &mut rec) {
        (rec.n + Vec3n::one()) * 0.5
    } else {
        let unit_direction = r.get_direction().normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::one() * (1.0 - t) + Vec3n::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let mut scene = HittableScene::<f32>::new();
    scene.add_hittable(Box::new(Sphere {
        center: Vec3n::new(0, 0, -1),
        radius: 0.5,
    }));
    scene.add_hittable(Box::new(Sphere {
        center: Vec3n::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    //    scene.add_hittable(Box::new(Sphere {
    //        center: Vec3n::new(-0.5, 0.0, -1.0),
    //        radius: 0.5,
    //    }));
    //    scene.add_hittable(Box::new(Sphere {
    //        center: Vec3n::new(0.5, 0.0, -1.0),
    //        radius: 0.5,
    //    }));

    let mut rng = rand::thread_rng();

    let samples = 100;

    let image_width = 400;
    let camera = Camera::<f32>::new(4.0, 4.0, 1.0, Vec3n::new(0, 0, 0));
    let image_height = (image_width as f32 * camera.aspect_ratio) as usize;

    let mut pixels = Vec::with_capacity(image_width * image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Vec3n::zero();
            for sample in 0..samples {
                let u = (i as f32 + rng.gen_range(0.0, 1.0)) / image_width as f32;
                let v = (j as f32 + rng.gen_range(0.0, 1.0)) / image_height as f32;
                let r = camera.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                color += get_ray_color(&r, &scene);
            }
            color /= samples as f32;

            pixels.push(color);
        }
    }
    write_ppm_file(&pixels, image_width, image_height, None);
}
