extern crate ray_trace;

use std::io;     
use std::io::prelude::*; 
use ray_trace::vector::*;
use ray_trace::ray::*;


pub fn ray_color(r: Ray) -> Vec3{
    let unit_direction: Vec3 = unit_vector(r.m);
    let t: f64 = 0.5 * (unit_direction.1 + 1.0);

    add(scale(1.0 - t, Vec3::new(1.0, 1.0, 1.0)), scale(t, Vec3::new(0.5, 0.7, 1.0)))
}
pub fn ppm_image(b: Vec3, cor: Vec3, hor: Vec3, ver: Vec3) {
    const WIDTH:i16 = 256;
    const HEIGHT:i16 = 256;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..(HEIGHT)).rev() {
        //println!("\rScanlines remaining: {} ", j);
        //io::stdout().flush().ok().expect("Could not flush stdout");
        for i in 0..WIDTH {
            let u: f64 = i as f64 / (WIDTH - 1) as f64;
            let v: f64 = j as f64 / (HEIGHT -1) as f64;
            let m: Vec3 = sub(add(cor, add(scale(u, hor), scale(v, ver))), b);
            let r: Ray = Ray::new(b, m);
            let color: Vec3 = ray_color(r);
            println!("{:.3} {:.3} {:.3}", color.0, color.1, color.2); 
        }
    }
    //println!("\nDone");
}

fn main() {
    //Image
    let aspect_ratio: f64 = 16.0/9.0;
    let image_width: i32 = 300;
    let image_height: i32 = image_width / aspect_ratio as i32;

    //Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Vec3 = Vec3::new(0., 0., 0.);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0., 0.);
    let vertical: Vec3 = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner: Vec3 = sub(origin, sub(sub(scale(0.5, horizontal), scale(0.5, vertical)), Vec3::new(0., 0., focal_length)));

    //ray_trace::image::ppm_image(); //cargo run > image.ppn | use google PPM viewer

    //Render
    ppm_image(origin, lower_left_corner, horizontal, vertical);
    
}
