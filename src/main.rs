use std::rc::Rc;
use crate::hittable::{hit_record, Hittable};
use crate::vec3::Point3;

mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod color;
mod rtweekend;
mod camera;

fn ray_color(r: &ray::Ray, world: &dyn Hittable, depth: usize) -> vec3::Color {
    let mut rec = hit_record::new();

    if depth <= 0 {
        return vec3::Color::new_with_values(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = (rec.p + rec.normal + vec3::random_unit_vector()) as Point3;
        return ray_color(&ray::Ray::new_with_values(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    vec3::Color::new_with_values(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Color::new_with_values(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Point3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = vec3::Vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (a);
    }
}


fn main() {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64/ ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 80;
    const MAX_DEPTH: usize = 20;
    eprintln!("Image size: {}x{} and aspect ratio: {}", IMAGE_WIDTH, IMAGE_HEIGHT, ASPECT_RATIO);

    // World
    let mut world = hittable_list::HittableList::new();
    world.add(Rc::new(sphere::Sphere::new_with_values(vec3::Point3::new_with_values(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(sphere::Sphere::new_with_values(vec3::Point3::new_with_values(0.0, -100.5, -1.0), 100.0)));



    //Camera

    let cam = camera::Camera::new();

    //Render

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = vec3::Color::new_with_values(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rtweekend::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rtweekend::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL as i32);
        }
    }

    eprintln!("\nDone.\n");
}
