use crate::{hittable, ray, vec3};

pub(crate) trait Material {
    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::hit_record, attenuation: &mut vec3::Color, scattered: &mut ray::Ray) -> bool;
}

pub struct Lambertian {
    albedo: vec3::Color,
}

impl Lambertian{
    fn new() -> Lambertian {
        Lambertian {
            albedo: vec3::Color::new_with_values(0.0, 0.0, 0.0),
        }
    }
    pub(crate) fn new_with_values(a: vec3::Color) -> Lambertian {
        Lambertian {
            albedo: a,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &ray::Ray, rec: &hittable::hit_record, attenuation: &mut vec3::Color, scattered: &mut ray::Ray) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = ray::Ray::new_with_values(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub(crate) struct Metal {
    albedo: vec3::Color,
    fuzz: f64,
}

impl Metal {
    fn new() -> Metal {
        Metal {
            albedo: vec3::Color::new_with_values(0.0, 0.0, 0.0),
            fuzz: 0.0,
        }
    }
    pub(crate) fn new_with_values(a: vec3::Color, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::hit_record, attenuation: &mut vec3::Color, scattered: &mut ray::Ray) -> bool {
        let reflected = vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = ray::Ray::new_with_values(rec.p, reflected + vec3::random_in_unit_sphere()*self.fuzz);
        *attenuation = self.albedo;

        vec3::Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

