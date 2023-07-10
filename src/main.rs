
mod vec3;


fn main() {

    // Image

    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    //Render

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = vec3::Color::new_with_values(i as f64 / (IMAGE_WIDTH - 1) as f64, j as f64 / (IMAGE_HEIGHT - 1) as f64, 0.25);
            vec3::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.\n");
}
