use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn main() {

    // Image

    let image_width: f64= 256f64;
    let image_height: f64 = 256f64;

    // Render
    let image_path = "./images/image.ppm";

    let mut image_file = File::create(image_path).expect("File creation failed!");

    // Write Header to File
    write!(image_file, "P3\n").expect("Write to file failed!");
    write!(image_file, "{}", image_width).expect("Write to file failed!");
    write!(image_file, " ").expect("Write to file failed!");
    write!(image_file, "{}", image_height).expect("Write to file failed!");
    write!(image_file, "\n255\n").expect("Write to file failed!");

    for j in (0..(image_height-1.0f64) as i16).rev() {
        for i in 0..(image_width as i16) {
            let r = i as f64/ (image_width-1.);
            let g = j as f64/ (image_height-1.);
            let b = 0.25;

            let ir: i16 = (255.999 * r as f64) as i16;
            let ig: i16 = (255.999 * g as f64) as i16;
            let ib: i16 = (255.999 * b as f64) as i16;

            write!(image_file, "{}", ir).expect("Write to file failed!");
            write!(image_file, " ").expect("Write to file failed!");
            write!(image_file, "{}", ig).expect("Write to file failed!");
            write!(image_file, " ").expect("Write to file failed!");
            write!(image_file, "{}", ib).expect("Write to file failed!");
            write!(image_file, "\n").expect("Write to file failed!");
        }
    }
}