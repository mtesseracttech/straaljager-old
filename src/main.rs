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

fn main() {
    //Setting up the scene and camera
    let offset = Vec3h::all(100); //This mostly exists to see if certain issues are caused by being near 0,0,0
    let camera = Camera::<f64>::new(4.0, 2.0, 1.0, Vec3h::new(0.0, 0.0, 0.0) + offset);
    let scene = Arc::new(set_up_scene(offset));

    //Setting up the output image settings
    let samples = 10;
    let image_width = 1000;
    let image_height = (image_width as f64 * camera.aspect_ratio) as usize;

    //Creating a carthesian product of the coordinates
    let coords: Vec<(usize, usize)> = (0..image_height)
        .rev()
        .map(move |j| (0..image_width).map(move |i| (i, j)))
        .flatten()
        .collect();

    //Frame buffer
    let mut pixels = Vec::with_capacity(image_width * image_height);

    //Parallel iteration over the coordinates, filling the frame buffer
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

    write_ppm_file(&pixels, image_width, image_height, None);
}

fn set_up_scene(offset: Vec3h) -> HittableScene<f64> {
    let mut scene = HittableScene::<f64>::new();

    scene.add_hittable(Arc::new(Sphere {
        center: Vec3h::new(0, 0, -1) + offset,
        radius: 0.5,
        material: Arc::new(LambertianMaterial {
            albedo: Vec3h::new(0.8, 0.3, 0.3),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3h::new(0.0, -100.5, -1.0) + offset,
        radius: 100.0,
        material: Arc::new(LambertianMaterial {
            albedo: Vec3h::new(0.8, 0.8, 0.0),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3h::new(1.0, 0.0, -1.0) + offset,
        radius: 0.5,
        material: Arc::new(MetalMaterial {
            albedo: Vec3h::new(0.8, 0.6, 0.2),
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

    scene
}

pub fn get_ray_color(r: &Ray<f64>, scene: &HittableScene<f64>, depth: u32) -> Vec3h {
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
