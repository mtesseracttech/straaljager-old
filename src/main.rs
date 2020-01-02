use std::sync::Arc;
use std::time::Instant;

use rand::Rng;
use rayon::prelude::*;
use straal::*;

use crate::geometry::*;
use crate::geometry::Hittable;
use crate::io::*;
use crate::material::*;
use crate::math::*;

pub mod geometry;
pub mod io;
pub mod material;
pub mod math;
pub mod textures;

type Precision = f32;

fn main() {
    //Timer
    let start_time = Instant::now();

    let mut scene = set_up_scene();
    let bvh = BvhNode::<Precision>::new(&mut scene.hittable_list[..], 0.0, 1.0);

    //Setting up the output image settings
    let samples = 50;
    let image_width = 600;
    let image_height = 480;

    let camera_pos = Vec3::new(8, 2, 3);
    let camera_target = Vec3::new(0.0, 0.0, 0.0);
    let focus_distance = Vec3::distance(camera_pos, camera_target);
    let aperture = 0.2;

    let camera = Camera::<Precision>::new(
        camera_pos,
        camera_target,
        Vec3::up(),
        40.0,
        image_width as Precision / image_height as Precision,
        aperture,
        focus_distance,
        0.0,
        1.0,
    );
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
                            let u = (i as Precision + rng.gen_range(-0.5, 0.5))
                                / image_width as Precision;
                            let v = (*j as Precision + rng.gen_range(-0.5, 0.5))
                                / image_height as Precision;
                            get_ray_color(&camera.get_ray(u, v), &bvh, 0)
                        })
                        .sum();
                    let res = average / samples as Precision;
                    gamma_color(&res)
                })
                .collect();
            println!("Row {} done", j);
            row
        })
        .collect_into_vec(&mut rows);

    //Frame buffer
    let mut pixels = Vec::with_capacity(image_width * image_height);

    for mut row in rows {
        pixels.append(&mut row);
    }

    println!("{}", duration_to_string(&start_time.elapsed()));

    write_ppm_file(&pixels, image_width, image_height, None);
}

fn set_up_scene() -> HittableScene<Precision> {
    let mut scene = HittableScene::<Precision>::new();

    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(0.0, -1000.0, -0.0),
        radius: 1000.0,
        material: Arc::new(LambertianMaterial {
            albedo: Vec3::<Precision>::new(0.5, 0.5, 0.5),
        }),
    }));

    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(-0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(DielectricMaterial {
            refractive_index: 1.5,
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(LambertianMaterial {
            albedo: Vec3::<Precision>::new(0.8, 0.3, 0.3),
        }),
    }));
    scene.add_hittable(Arc::new(Sphere {
        center: Vec3::<Precision>::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(MetalMaterial {
            albedo: Vec3::<Precision>::new(0.8, 0.6, 0.2),
            roughness: 0.8,
        }),
    }));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let r = 0.2;
            let mat_choice = rng.gen_range(0.0, 1.0);
            let c = Vec3::new(
                a as Precision + rng.gen_range(0.0, 1.0 - r / 2.0),
                r,
                b as Precision + rng.gen_range(0.0, 1.0 - r / 2.0),
            ) * Vec3::new(1.0, 1.0, 1.0);
            if mat_choice < 0.8 {
                scene.add_hittable(Arc::new(MovableSphere {
                    center0: c,
                    center1: c + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0),
                    time0: 0.0,
                    time1: 1.0,
                    radius: r,
                    material: Arc::new(LambertianMaterial {
                        albedo: Vec3::<Precision>::new(
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                        ),
                    }),
                }));
            } else if mat_choice < 0.95 {
                scene.add_hittable(Arc::new(Sphere {
                    center: c,
                    radius: r,
                    material: Arc::new(MetalMaterial {
                        albedo: Vec3::<Precision>::new(
                            rng.gen_range(0.5, 1.0),
                            rng.gen_range(0.5, 1.0),
                            rng.gen_range(0.5, 1.0),
                        ),
                        roughness: (rng.gen_range(0.0, 0.5)),
                    }),
                }));
            } else {
                scene.add_hittable(Arc::new(Sphere {
                    center: c,
                    radius: r,
                    material: Arc::new(DielectricMaterial {
                        refractive_index: 1.5,
                    }),
                }));
            }
        }
    }

    println!("Scene set up.");
    scene
}

pub fn get_ray_color(
    r: &Ray<Precision>,
    scene: &BvhNode<Precision>,
    depth: u32,
) -> Vec3<Precision> {
    let mut rec = HitRecord::<Precision>::default();
    if scene.hit(r, 0.01, 10000000.0, &mut rec) {
        let mut scattered = Ray::<Precision>::default();
        let mut attenuation = Vec3::<Precision>::zero();
        if depth < 50 && rec.material.upgrade().expect("Could not get RC to material from weak ptr").scatter(r, &mut rec, &mut attenuation, &mut scattered) {
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
