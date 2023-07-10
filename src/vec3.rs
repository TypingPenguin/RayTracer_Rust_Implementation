use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};

type Point3 = Vec3;
pub(crate) type Color = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e:[f64; 3],
}

impl Vec3 {
    fn new() -> Self {
        Vec3{e:[0.0, 0.0, 0.0]}
    }

    pub(crate) fn new_with_values(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3{e:[e0, e1, e2]}
    }

    fn x(&self) -> f64 {
        self.e[0]
    }

    fn y(&self) -> f64 {
        self.e[1]
    }

    fn z(&self) -> f64 {
        self.e[2]
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    fn dot(u: Vec3, v: Vec3) -> f64 {
        u.e[0]*v.e[0] + u.e[1]*v.e[1] + u.e[2]*v.e[2]
    }

    fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3::new_with_values(u.e[1]*v.e[2] - u.e[2]*v.e[1],
                              u.e[2]*v.e[0] - u.e[0]*v.e[2],
                              u.e[0]*v.e[1] - u.e[1]*v.e[0])
    }

    fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new_with_values(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3::new_with_values(self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new_with_values(self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Vec3::new_with_values(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Self::Output {
        Vec3::new_with_values(self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2])
    }
}

impl Div<f64> for Vec3{
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        Vec3::new_with_values(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new_with_values(self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2])
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub(crate) fn write_color(pixel_color: Color) {
    println!("{} {} {}", (255.999 * pixel_color.x()) as i32, (255.999 * pixel_color.y()) as i32, (255.999 * pixel_color.z()) as i32);
}
