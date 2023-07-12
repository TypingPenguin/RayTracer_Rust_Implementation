use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::rtweekend::degrees_to_radians;
use crate::vec3;


pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3,lookat: Point3,vup: Point3,vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 -  w * focus_dist;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,}
    }

    pub(crate) fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();


        Ray::new_with_values(self.origin +offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset)
    }
}