use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::rtweekend::degrees_to_radians;


pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3,lookat: Point3,vup: Point3,vfov: f64, aspect_ratio: f64) -> Self {

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub(crate) fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new_with_values(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}