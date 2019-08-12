use std::sync::Arc;
use std::time::Instant;

use rand::Rng;
use rayon::prelude::*;
use straal::*;

use crate::geometry::*;
use crate::io::*;
use crate::material::*;
use crate::math::*;

pub mod geometry;
pub mod io;
pub mod material;
pub mod math;

type Precision = f64;

fn main() {
    //Timer
    let start_time = Instant::now();

    let scene = Arc::new(set_up_scene());

    //Setting up the output image settings
    let samples = 100;
    let image_width = 600;
    let image_height = 300;

    let camera_pos = Vec3::new(3, 3, 2);
    let camera_target = Vec3::new(0, 0, -1);
    let focus_distance = Vec3::distance(camera_pos, camera_target);
    let aperture = 2.0;

    let camera = Camera::<Precision>::new(camera_pos,
                                          camera_target,
                                          Vec3::new(0, 1, 0),
                                          20.0,
                                          image_width as Precision / image_height as Precision,
                                          aperture,
                                          focus_distance);

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

fn set_up_scene() -> HittableScene<Precision> {
    let mut scene = HittableScene::<Precision>::new();

    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(0.0, -1000.5, -1.0),
        radius: 1000.0,
        material: Arc::new(LambertianMaterial {
            albedo: Vec3::<Precision>::new(0.5, 0.5, 0.5),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(LambertianMaterial {
            albedo: Vec3::<Precision>::new(0.8, 0.3, 0.3),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(MetalMaterial {
            albedo: Vec3::<Precision>::new(0.8, 0.6, 0.2),
            roughness: 0.5,
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(-1.0, 0.0, -1.0),
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
        let mut scattered = Ray::<Precision>::default();
        let mut attenuation = Vec3::<Precision>::zero();
        if depth < 50
            && rec
            .material
            .upgrade()
            .expect("Could not get RC to material from weak ptr")
            .scatter(r, &mut rec, &mut attenuation, &mut scattered)
        {
            attenuation * get_ray_color(&scattered, &scene, depth + 1)
        } else {
            Vec3::<Precision>::zero()
        }
    } else {
        let unit_direction = r.get_direction().normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::<Precision>::one() * (1.0 - t) + Vec3::<Precision>::new(0.5, 0.7, 1.0) * t
    }
}
