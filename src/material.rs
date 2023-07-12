use crate::{hittable, ray, vec3};
use crate::hittable::hit_record;
use crate::ray::Ray;
use crate::vec3::Color;

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

pub(crate) struct Dielectric {
    ir: f64,
}

impl Dielectric {
    fn new() -> Dielectric {
        Dielectric {
            ir: 0.0,
        }
    }
    pub(crate) fn new_with_values(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new_with_values(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = r_in.direction().unit_vector();

        let cos_theta = f64::min(vec3::Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random() {
                vec3::reflect(&unit_direction, &rec.normal)
            } else {
                vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        *scattered = Ray::new_with_values(rec.p, direction);
        true
    }

}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
}