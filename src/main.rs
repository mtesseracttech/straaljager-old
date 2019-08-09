pub mod geometry;
pub mod io;
pub mod material;
pub mod math;

use crate::geometry::*;
use crate::io::*;
use crate::material::*;
use crate::math::*;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use straal::*;

type Precision = f64;

fn main() {
    //Timer
    let start_time = Instant::now();

    //Setting up the scene and camera
    let offset = Vec3::<Precision>::all(100); //This mostly exists to see if certain issues are caused by being near 0,0,0
    let camera = Camera::<Precision>::new(
        4.0,
        2.0,
        1.0,
        Vec3::<Precision>::new(0.0, 0.0, 0.0) + offset,
    );
    let scene = Arc::new(set_up_scene(offset));

    //Setting up the output image settings
    let samples = 100;
    let image_width = 1000;
    let image_height = (image_width as Precision * camera.aspect_ratio) as usize;

    let row_coords: Vec<usize> = (0..image_height).rev().collect();

    let mut rows: Vec<Vec<Vec3<Precision>>> = Vec::with_capacity(image_height);
    row_coords
        .par_iter()
        .map(|j| {
            let mut rng = rand::thread_rng();
            let row: Vec<Vec3<Precision>> = (0..image_width)
                .map(|i| {
                    let average: Vec3<Precision> = (0..samples)
                        .map(|_s| {
                            let u = (i as Precision + rng.gen_range(0.0, 1.0))
                                / image_width as Precision;
                            let v = (*j as Precision + rng.gen_range(0.0, 1.0))
                                / image_height as Precision;
                            get_ray_color(&camera.get_ray(u, v), &scene, 0)
                        })
                        .sum();
                    let res = average / samples as Precision;
                    gamma_color(&res)
                })
                .collect();
            row
        })
        .collect_into_vec(&mut rows);

    //Frame buffer
    let mut pixels = Vec::with_capacity(image_width * image_height);

    for mut row in rows {
        pixels.append(&mut row);
    }

    let end_time = start_time.elapsed();
    println!(
        "Time taken for ray tracing: {:.4}s",
        end_time.as_secs() as f64 + end_time.subsec_nanos() as f64 / 1.0e+9,
    );
    write_ppm_file(&pixels, image_width, image_height, None);
}

fn set_up_scene(offset: Vec3<Precision>) -> HittableScene<Precision> {
    let mut scene = HittableScene::<Precision>::new();

    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(0, 0, -1) + offset,
        radius: 0.5,
        material: Arc::new(LambertianMaterial {
            albedo: Vec3::<Precision>::new(0.8, 0.3, 0.3),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(0.0, -100.5, -1.0) + offset,
        radius: 100.0,
        material: Arc::new(LambertianMaterial {
            albedo: Vec3::<Precision>::new(0.8, 0.8, 0.0),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(1.0, 0.0, -1.0) + offset,
        radius: 0.5,
        material: Arc::new(MetalMaterial {
            albedo: Vec3::<Precision>::new(0.8, 0.6, 0.2),
            roughness: 1.0,
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(-1.0, 0.0, -1.0) + offset,
        radius: 0.5,
        material: Arc::new(DielectricMaterial {
            refractive_index: 2.4,
        }),
    }));

    scene
}

pub fn get_ray_color(
    r: &Ray<Precision>,
    scene: &HittableScene<Precision>,
    depth: u32,
) -> Vec3<Precision> {
    let mut rec = HitRecord::<Precision>::default();
    if scene.hit(r, 0.0, 10000000.0, &mut rec) {
        let mut scattered = Ray::<Precision> {
            origin: Vec3::<Precision>::zero(),
            direction: Vec3::<Precision>::zero(),
        };
        let mut attenuation = Vec3::<Precision>::zero();
        if depth < 50
            && rec
                .material
                .upgrade()
                .expect("Could not get RC to material from weak ptr")
                .scatter(r, &mut rec, &mut attenuation, &mut scattered)
        {
            attenuation * get_ray_color(&mut scattered, scene, depth + 1)
        } else {
            Vec3::<Precision>::zero()
        }
    } else {
        let unit_direction = r.get_direction().normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::<Precision>::one() * (1.0 - t) + Vec3::<Precision>::new(0.5, 0.7, 1.0) * t
    }
}
