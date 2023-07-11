

use crate::vec3::Color;
use std::io::Write;
use crate::rtweekend::clamp;


pub(crate) fn write_color<W: Write>(out: &mut W, pixel_color: Color, samples_per_pixel: i32) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();


    writeln!(out,"{} {} {}", (256.00 * clamp(r, 0.0, 0.999)) as i32 , (256.00 * clamp(g, 0.0, 0.999)) as i32, (256.00 * clamp(b, 0.0, 0.999)) as i32).expect("Error writing color");
}