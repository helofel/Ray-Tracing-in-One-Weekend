extern crate ray_trace;

use std::io;     
use std::io::prelude::*; 
use ray_trace::vector::*;
use ray_trace::ray::*;

pub fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc: Vec3 = sub(r.b, center);
    let a: f64 = dot(r.m, r.m);
    let b: f64 = 2.0 * dot(oc, r.m);
    let c: f64 = dot(oc, oc) - radius * radius;
    let discriminant: f64 = b * b - 4. * a * c;

    if discriminant < 0. {
        return -1.0;
    }

    (-b - discriminant.sqrt() ) / (2.0 * a)
}

pub fn ray_color(r: Ray) -> Vec3{
    let mut t: f64 = hit_sphere(Vec3::new(0., 0., -1.), 0.5, r);

    if t > 0.0 {
        let n: Vec3 = unit_vector(sub(at(r, t), Vec3::new(0., 0., -1.)));
        return scale(0.5, Vec3::new(n.x() + 1., n.y() + 1., n.z() + 1.));
    }

    let unit_direction: Vec3 = unit_vector(r.m);
    t = 0.5 * (unit_direction.y() + 1.);

    add(scale(1.0 - t, Vec3::new(1., 1., 1.)), scale(t, Vec3::new(0.5, 0.7, 1.)))
}

pub fn ppm_image(image_width: i32, image_height: i32, b: Vec3, cor: Vec3, hor: Vec3, ver: Vec3) {
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        //println!("\rScanlines remaining: {} ", j);
        io::stdout().flush().ok().expect("Could not flush stdout");
        for i in 0..image_width {
            let u: f64 = i as f64 / (image_width - 1) as f64;
            let v: f64 = j as f64 / (image_height -1) as f64;
            let m: Vec3 = sub(add(cor, add(scale(u, hor), scale(v, ver))), b);
            let r: Ray = Ray::new(b, m);
            let color: Vec3 = ray_color(r);
            println!("{} {} {}", 255.999 * color.0, 255.999 * color.1, 255.999 * color.2); 
        }

    }
    //println!("\nDone");
}

fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = image_width / aspect_ratio as i32;

    //Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Vec3 = Vec3::new(0., 0., 0.);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0., 0.);
    let vertical: Vec3 = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner: Vec3 = sub(origin, sub(sub(scale(0.5, horizontal), scale(0.5, vertical)), Vec3::new(0., 0., focal_length)));

    //ray_trace::image::ppm_image(); //cargo run > image.ppm

    //Render
    ppm_image(image_width, image_height, origin, lower_left_corner, horizontal, vertical);
}
