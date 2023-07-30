use std::rc::Rc;
use crate::hittable::{hit_record, Hittable};
use crate::hittable_list::HittableList;
use crate::vec3::Point3;

mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod color;
mod rtweekend;
mod camera;
mod material;

fn ray_color(r: &ray::Ray, world: &dyn Hittable, depth: usize) -> vec3::Color {
    let mut rec = hit_record::new();

    if depth <= 0 {
        return vec3::Color::new_with_values(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = ray::Ray::new();
        let mut attenuation = vec3::Color::new_with_values(0.0, 0.0, 0.0);
        if rec.mat_ptr.as_ref().unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return vec3::Color::new_with_values(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    vec3::Color::new_with_values(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Color::new_with_values(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Point3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = vec3::Vec3::dot(&oc, &r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (a);
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(material::Lambertian::new_with_values(vec3::Color::new_with_values(0.5, 0.5, 0.5)));
    world.add(Rc::new(sphere::Sphere::new_with_values(Point3::new_with_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new_with_values(a as f64 + 0.9*rtweekend::random_double(), 0.2, b as f64 + 0.9*rtweekend::random_double());
            if (center - Point3::new_with_values(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn material::Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::random() * vec3::random();
                    sphere_material = Rc::new(material::Lambertian::new_with_values(albedo));
                    world.add(Rc::new(sphere::Sphere::new_with_values(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::random_minmax(0.5, 1.0);
                    let fuzz = rtweekend::random_double_minmax(0.0, 0.5);
                    sphere_material = Rc::new(material::Metal::new_with_values(albedo, fuzz));
                    world.add(Rc::new(sphere::Sphere::new_with_values(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(material::Dielectric::new_with_values(1.5));
                    world.add(Rc::new(sphere::Sphere::new_with_values(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(material::Dielectric::new_with_values(1.5));
    world.add(Rc::new(sphere::Sphere::new_with_values(Point3::new_with_values(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(material::Lambertian::new_with_values(vec3::Color::new_with_values(0.4, 0.2, 0.1)));
    world.add(Rc::new(sphere::Sphere::new_with_values(Point3::new_with_values(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(material::Metal::new_with_values(vec3::Color::new_with_values(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(sphere::Sphere::new_with_values(Point3::new_with_values(4.0, 1.0, 0.0), 1.0, material3)));

    world





}



fn main() {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 1920;
    const SAMPLES_PER_PIXEL: usize = 30;
    const MAX_DEPTH: usize = 5;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64/ ASPECT_RATIO) as usize;
    eprintln!("Image size: {}x{} and aspect ratio: {}", IMAGE_WIDTH, IMAGE_HEIGHT, ASPECT_RATIO);

    // World
    let mut world = random_scene();

    //Camera

    let lookfrom = vec3::Point3::new_with_values(13.0, 2.0, 3.0);
    let lookat = vec3::Point3::new_with_values(0.0, 0.0, 0.0);
    let vup = vec3::Vec3::new_with_values(0.0, 1.0, 0.0);
    let dist_to_focus = 10 as f64;
    let aperture = 0.1;

    let cam = camera::Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

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
