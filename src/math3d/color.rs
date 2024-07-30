use std::io;

use crate::math3d::Vector3;

pub type Color = Vector3;

pub fn write_color(mut out: impl io::Write, pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    out.write_fmt(format_args!("{} {} {}\n", rbyte, gbyte, bbyte))
        .unwrap();
}
