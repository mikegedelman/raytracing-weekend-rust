use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

#[inline]
pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

#[inline]
pub fn random_f32_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[inline]
pub fn random_usize(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
