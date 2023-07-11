use crate::vec3;
use crate::ray;


struct hit_record {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl hit_record {
    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: hit_record) -> bool;
}

