mod math3d;
use math3d::{write_color, Color};

fn main() {
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color::new(
                f64::from(i) / f64::from(image_width - 1),
                f64::from(j) / f64::from(image_height - 1),
                0.0,
            );
            write_color(std::io::stdout(), &pixel_color);
        }
    }
}
