use straal::{Vec3, FloatType};
use std::path::Path;
use std::io::{BufReader, Write};
use std::fs::File;
use std::fs;
use std::time::SystemTime;
use std::error::Error;
use rand::{self, Rng};

pub struct Ray<T>{
    pub o : Vec3<T>,
    pub d : Vec3<T>
}

impl<T> Ray<T> where T : FloatType<T>{
    fn get_origin(&self) -> Vec3<T> {

    }
}

fn main() {
    let n_x = 200;
    let n_y = 100;
    let mut output = String::with_capacity(20 + n_x * n_y * 12); //Assumed max size of output file
    let header = format!("P3\n{} {}\n255\n", n_x, n_y);
    output.push_str(&header);
    for j in (0..n_y).rev() {
        for i in 0..n_x {
            let col = Vec3{
                x: i as f32 / n_x as f32,
                y: j as f32 / n_y as f32,
                z: 0.2
            };
            let i_r = (255.99 * col[0]) as i32;
            let i_g = (255.99 * col[1]) as i32;
            let i_b = (255.99 * col[2]) as i32;
            let line = format!("{} {} {}\n", i_r, i_g, i_b);
            output.push_str(&line);
        }
    }
    write_ppm_file(&output);
}

fn write_ppm_file(output : &str){
    let unique_id : u32 = rand::thread_rng().gen_range(0, 999999);
    let file_name = format!("output_{:0>6}.ppm", unique_id);
    let file_path = "./output/";
    match File::create(file_path.to_owned() + &file_name) {
        Ok(mut file) => {
            match file.write_all(&output.as_bytes()){
                Ok(s) => { println!("Succeeded in writing file"); },
                Err(e) => {println!("{}", e);},
            }
        },
        Err(e) => { panic!("Could not create file: /n{}", e.description() )
        },
    }
}

