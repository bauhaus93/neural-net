use std::f32;
use std::f32::consts::PI;

pub const PI_DOUBLE: f32 = PI * 2.0;
pub const PI_HALF: f32 = PI / 2.0;

pub enum Boundary {
    HORIZONTAL, VERTICAL
}

pub trait Vector2D {
    fn normalize(&self) -> Self;
    fn length(&self) -> f32;
    fn get_angle_diff(&self, other: &Self) -> f32;
    fn add(&self, other: &Self) -> Self;
}

impl Vector2D for (f32, f32) {
    fn normalize(&self) -> Self {
        let len = self.length();
        (self.0 / len, self.1 / len)
    }

    fn length(&self) -> f32 {
        f32::sqrt(self.0 * self.0 + self.1 * self.1)
    }

    fn get_angle_diff(&self, other: &Self) -> f32 {
        let a_norm = self.normalize();
        let b_norm = other.normalize();

        let mut angle = f32::atan2(b_norm.1, b_norm.0) - f32::atan2(a_norm.1, a_norm.0);
        if angle > PI {
            angle -= PI_DOUBLE;
        }
        else if angle < -PI {
            angle += PI_DOUBLE;
        }
        angle
    }

    fn add(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }
}

/*pub fn get_vector_length(v: (f32, f32)) -> f32 {
    f32::sqrt(v.0 * v.0 + v.1 * v.1)
}*/

pub fn get_distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dist_x = a.0 - b.0;
    let dist_y = a.1 - b.1;
    f32::sqrt(dist_x * dist_x + dist_y * dist_y)
}

/*pub fn normalize(v: (f32, f32)) -> (f32, f32) {
    let len = get_vector_length(v);
    (v.0 / len, v.1 / len)
}

pub fn get_angle(a: (f32, f32), b: (f32, f32)) -> f32 {
    let a_norm = a.normalize();//normalize(a);
    let b_norm = b.normalize();//normalize(b);

    let mut angle = f32::atan2(b_norm.1, b_norm.0) - f32::atan2(a_norm.1, a_norm.0);
    if angle > PI {
        angle -= PI_DOUBLE;
    }
    else if angle < -PI {
        angle += PI_DOUBLE;
    }
    angle
}*/
