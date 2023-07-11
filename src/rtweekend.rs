use rand::Rng;

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Random f64 in [0, 1)
pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

// Random f64 in [min, max)
pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}