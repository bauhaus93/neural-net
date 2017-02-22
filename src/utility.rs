use std::f32;
use std::f32::consts::PI;

pub const PI_DOUBLE: f32 = PI * 2.0;
pub const PI_HALF: f32 = PI / 2.0;

pub enum Boundary {
    HORIZONTAL, VERTICAL
}

pub fn get_vector_length(v: (f32, f32)) -> f32 {
    f32::sqrt(v.0 * v.0 + v.1 * v.1)
}

pub fn get_distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dist_x = a.0 - b.0;
    let dist_y = a.1 - b.1;
    f32::sqrt(dist_x * dist_x + dist_y * dist_y)
}

pub fn normalize(v: (f32, f32)) -> (f32, f32) {
    let len = get_vector_length(v);
    (v.0 / len, v.1 / len)
}

pub fn get_angle(a: (f32, f32), b: (f32, f32)) -> f32 {
    let a_norm = normalize(a);
    let b_norm = normalize(b);

    let mut angle = f32::atan2(b_norm.1, b_norm.0) - f32::atan2(a_norm.1, a_norm.0);
    if angle > PI {
        angle -= PI_DOUBLE;
    }
    else if angle < -PI {
        angle += PI_DOUBLE;
    }
    angle
}
