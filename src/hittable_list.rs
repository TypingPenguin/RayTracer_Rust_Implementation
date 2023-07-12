use std::rc::Rc;
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

    pub(crate) fn add(&mut self, object: Rc<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, mut rec: &mut hittable::hit_record) -> bool {
        let mut temp_rec = hittable::hit_record::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                let temp_rec_clone = temp_rec.clone();
                closest_so_far = temp_rec_clone.t;
                *rec = temp_rec_clone;
            }
        }

        hit_anything
    }
}
