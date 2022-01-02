use std::io;     
use std::io::prelude::*; 

use crate::vector::*;

pub fn ppm_image() {
    const WIDTH:i16 = 256;
    const HEIGHT:i16 = 256;
    let mut color: Vec3;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..(HEIGHT)).rev() {
        println!("\rScanlines remaining: {} ", j);
        io::stdout().flush().ok().expect("Could not flush stdout");
        for i in 0..WIDTH {
            //Generate a PPN image
            /*
            let r:f64 = i as f64 / (WIDTH - 1) as f64;
            let g:f64 = j as f64 / (HEIGHT - 1) as f64;
            let b:f64 = 0.25;

            let ir: i16 = (255.999 * r) as i16;
            let ig: i16 = (255.999 * g) as i16;
            let ib: i16 = (255.999 * b) as i16;
            println!("{} {} {}", ir, ig, ib);
            */
            color = Vec3(i as f64 / (WIDTH - 1) as f64, j as f64 / (HEIGHT - 1) as f64, 0.25);
            println!("{:.3} {:.3} {:.3}", color.0, color.1, color.2); 
        }
    }
    println!("\nDone");
}