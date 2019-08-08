pub mod camera;
pub mod hittable;
pub mod material;
pub mod ppm_file;
pub mod ray;
pub mod sphere;
pub mod vector_utils;

use rand::Rng;
use straal::{Vec3, Vec3h};

use camera::*;
use hittable::*;
use material::*;
use ppm_file::*;
use ray::*;
use rayon::prelude::*;
use sphere::*;
use std::sync::Arc;
use vector_utils::*;

pub fn get_ray_color(r: &Ray<f64>, scene: &HittableScene<f64>, depth: u32) -> Vec3<f64> {
    let mut rec = HitRecord::<f64>::default();
    if scene.hit(r, 0.0, std::f64::MAX, &mut rec) {
        let mut scattered = Ray::<f64> {
            origin: Vec3h::zero(),
            direction: Vec3h::zero(),
        };
        let mut attenuation = Vec3h::zero();
        if depth < 50
            && rec
                .material
                .upgrade()
                .expect("Could not get RC to material from weak ptr")
                .scatter(r, &mut rec, &mut attenuation, &mut scattered)
        {
            attenuation * get_ray_color(&mut scattered, scene, depth + 1)
        } else {
            Vec3h::zero()
        }
    } else {
        let unit_direction = r.get_direction().normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3h::one() * (1.0 - t) + Vec3h::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let offset = Vec3h::all(100);
    let camera = Camera::<f64>::new(4.0, 2.0, 1.0, Vec3h::new(0.0, 0.0, 0.0) + offset);

    let mut scene = HittableScene::<f64>::new();
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3h::new(0, 0, -1) + offset,
        radius: 0.5,
        material: Arc::new(LambertianMaterial::<f64> {
            albedo: Vec3::new(0.8, 0.3, 0.3),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3h::new(0.0, -100.5, -1.0) + offset,
        radius: 100.0,
        material: Arc::new(LambertianMaterial::<f64> {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3h::new(1.0, 0.0, -1.0) + offset,
        radius: 0.5,
        material: Arc::new(MetalMaterial {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            roughness: 1.0,
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3h::new(-1.0, 0.0, -1.0) + offset,
        radius: 0.5,
        material: Arc::new(DielectricMaterial {
            refractive_index: (1.0),
        }),
    }));

    let mut scene = Arc::new(scene);

    let samples = 10;

    let image_width = 1000;
    let image_height = (image_width as f64 * camera.aspect_ratio) as usize;

    let coords: Vec<(usize, usize)> = (0..image_height)
        .rev()
        .map(move |j| (0..image_width).map(move |i| (i, j)))
        .flatten()
        .collect();

    let mut pixels = Vec::with_capacity(image_width * image_height);

    coords
        .par_iter()
        .map(|(i, j)| {
            let mut rng = rand::thread_rng();
            let average: Vec3h = (0..samples)
                .map(|_s| {
                    let u = (*i as f64 + rng.gen_range(0.0, 1.0)) / image_width as f64;
                    let v = (*j as f64 + rng.gen_range(0.0, 1.0)) / image_height as f64;
                    get_ray_color(&camera.get_ray(u, v), &scene, 0)
                })
                .sum();
            let res = average / samples as f64;
            gamma_color(&res)
        })
        .collect_into_vec(&mut pixels);

    //    for j in (0..image_height).rev() {
    //        for i in (0..image_width) {
    //            let mut color = Vec3h::zero();
    //            for _sample in 0..samples {
    //                let u = (i as f64 + rng.gen_range(0.0, 1.0)) / image_width as f64;
    //                let v = (j as f64 + rng.gen_range(0.0, 1.0)) / image_height as f64;
    //                let r = camera.get_ray(u, v);
    //                color += get_ray_color(&r, &scene, 0);
    //            }
    //            color /= samples as f64;
    //
    //            pixels.push(color);
    //        }
    //    }

    //Gamma correction
    //pixels.par_iter().flat_map(|v| gamma_color(v));
    //    for i in 0..pixels.len() {
    //        //        if pixels[i].get_smallest() < 0.0 {
    //        //            println!("{}", pixels[i]);
    //        //            pixels[i] = Vec3h::new(1, 0, 0);
    //        //        } else {
    //        pixels[i] = gamma_color(&pixels[i]);
    //        //}
    //    }

    write_ppm_file(&pixels, image_width, image_height, None);
}
