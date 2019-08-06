use num::NumCast;
use rand::{self, Rng};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use straal::{FloatType, IVec3, Vec3, Vec3n};

pub fn to_ppm_color<T>(v: &Vec3<T>) -> IVec3<i32>
where
    T: FloatType<T>,
{
    let max = T::from(255.99).unwrap();
    IVec3 {
        x: num::cast(max * v.x).unwrap(),
        y: num::cast(max * v.y).unwrap(),
        z: num::cast(max * v.z).unwrap(),
    }
}

pub fn write_ppm_file(pixels: &Vec<Vec3n>, width: usize, height: usize, file_name: Option<&str>) {
    let real_file_name = match file_name {
        None => {
            let unique_id: u32 = rand::thread_rng().gen_range(0, 999999);
            format!("output_{:0>6}.ppm", unique_id)
        }
        Some(n) => n.to_string(),
    };

    let file_path = "./output/";

    let mut output = String::with_capacity(20 + pixels.len() * 12); //Assumed max size of output file
    output.push_str(&format!("P3\n{} {}\n255\n", width, height)); //Header

    for pixel in pixels {
        let ppm_color = to_ppm_color(&pixel);
        output.push_str(&format!(
            "{} {} {}\n",
            ppm_color.x, ppm_color.y, ppm_color.z
        ));
    }

    match File::create(file_path.to_owned() + &real_file_name) {
        Ok(mut file) => match file.write_all(&output.as_bytes()) {
            Ok(_s) => {
                println!("Succeeded in writing file");
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        Err(e) => panic!("Could not create file: /n{}", e.description()),
    }
}
