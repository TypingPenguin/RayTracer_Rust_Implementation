use crate::vec3;
use crate::ray;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct hit_record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl hit_record {
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
    pub fn new() -> hit_record {
        hit_record {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
        }
    }
}

pub(crate) trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool;
}

