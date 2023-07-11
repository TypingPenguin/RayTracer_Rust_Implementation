use crate::hittable;
use crate::vec3;
use crate::ray;

pub struct HittableList {
    objects: Vec<Rc<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, r: ray::Ray, t_min: f64, t_max: f64, rec: hittable::HitRecord) -> bool {
        let mut temp_rec = hittable::HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }

        hit_anything
    }
}
