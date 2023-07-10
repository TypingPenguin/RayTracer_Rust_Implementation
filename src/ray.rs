
use crate::vec3::{Point3, Vec3};

pub(crate) struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub(crate) fn new_with_values(origin: Point3, direction: Vec3) -> Ray {
        Ray { orig: origin, dir: direction }
    }

    fn origin(&self) -> Point3 {
        self.orig
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.dir
    }

    fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}