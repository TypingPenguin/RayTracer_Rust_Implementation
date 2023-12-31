use std::rc::Rc;
use crate::{hittable, material, vec3};
use crate::ray;


pub struct Sphere {
    center: vec3::Point3,
    radius: f64,
    mat_ptr: Rc<dyn material::Material>,
}

impl Sphere {
    fn new() -> Sphere {
        Sphere {
            center: vec3::Point3::new(),
            radius: 0.0,
            mat_ptr: Rc::new(material::Lambertian::new_with_values(vec3::Color::new_with_values(0.0, 0.0, 0.0))),
        }
    }

    pub(crate) fn new_with_values(center: vec3::Point3, radius: f64, material: Rc<dyn material::Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            mat_ptr: material,
        }
    }

}



impl hittable::Hittable for Sphere {

    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hittable::hit_record) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot_self(r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Option::from(Rc::clone(&self.mat_ptr));

        true
    }
}