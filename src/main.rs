use std::rc::Rc;
use crate::hittable::{hit_record, Hittable};
use crate::vec3::Point3;

mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;

fn ray_color(r: &ray::Ray, world: &dyn Hittable) -> vec3::Color {
    let mut rec = hit_record::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return (rec.normal + vec3::Color::new_with_values(1.0, 1.0, 1.0)) * 0.5;
    }
    //
    // let t = hit_sphere(&Point3::new_with_values(0.0, 0.0, -1.0), 0.5, r);
    // if t > 0.0 {
    //     let N = (r.at(t) - vec3::Vec3::new_with_values(0.0, 0.0, -1.0)).unit_vector();
    //     return vec3::Color::new_with_values(N.x()+1.0, N.y()+1.0, N.z()+1.0) * 0.5;
    // }

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
    eprintln!("Image size: {}x{} and aspect ratio: {}", IMAGE_WIDTH, IMAGE_HEIGHT, ASPECT_RATIO);

    // World
    let mut world = hittable_list::HittableList::new();
    world.add(Rc::new(sphere::Sphere::new_with_values(vec3::Point3::new_with_values(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(sphere::Sphere::new_with_values(vec3::Point3::new_with_values(0.0, -100.5, -1.0), 100.0)));



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
            let pixel_color = ray_color(&r, &world);
            vec3::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.\n");
}
