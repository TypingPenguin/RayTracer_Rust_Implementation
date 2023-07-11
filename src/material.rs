

pub trait Material {
    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord, attenuation: &mut vec3::Color, scattered: &mut ray::Ray) -> bool;
}