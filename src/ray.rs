
use crate::vec3::{Point3, Vec3};

pub(crate) struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray { orig: Point3::new(), dir: Vec3::new() }
    }

    pub(crate) fn new_with_values(origin: Point3, direction: Vec3) -> Ray {
        Ray { orig: origin, dir: direction }
    }

    pub(crate) fn origin(&self) -> Point3 {
        self.orig
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.dir
    }

    pub(crate) fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}