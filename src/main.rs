
mod vec3;
mod ray;

fn ray_color(r: &ray::Ray) -> vec3::Color {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    vec3::Color::new_with_values(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Color::new_with_values(0.5, 0.7, 1.0) * t
}



fn main() {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64/ ASPECT_RATIO) as usize;
    eprintln!("Image size: {}x{} and aspect ratio: {}", IMAGE_WIDTH, IMAGE_HEIGHT, ASPECT_RATIO);

    //Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::new_with_values(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new_with_values(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new_with_values(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new_with_values(0.0, 0.0, focal_length);

    //Render

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        // eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = ray::Ray::new_with_values(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(&r);
            vec3::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.\n");
}
