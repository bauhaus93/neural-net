use std::f32;
use std::f32::consts::PI;
use std::option::Option;
use std::marker::Sized;

pub const PI_DOUBLE: f32 = PI * 2.0;
pub const PI_HALF: f32 = PI / 2.0;

pub enum Boundary {
    HORIZONTAL, VERTICAL
}

pub trait Vector2D
    where Self: Sized {
    fn normalize(self) -> Self;
    fn length(&self) -> f32;
    fn get_angle_diff(self, other: Self) -> f32;
    fn is_clockwise(self, other: Self) -> bool;
    fn dot(self, other: Self) -> f32;
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
}

impl Vector2D for (f32, f32) {
    fn normalize(self) -> Self {
        let len = self.length();
        (self.0 / len, self.1 / len)
    }

    fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    fn get_angle_diff(self, other: Self) -> f32 {
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

    fn is_clockwise(self, other: Self) -> bool {
        !(self.1 * other.0 > self.0 * other.1)
    }

    fn dot(self, other: Self) -> f32 {
        self.0 * other.0 + self.1 * other.1
    }

    fn add(self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }

    fn sub(self, other: Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }
}

pub fn get_distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dist_x = a.0 - b.0;
    let dist_y = a.1 - b.1;
    f32::sqrt(dist_x * dist_x + dist_y * dist_y)
}

pub fn line_intersects_line(line_a: ((f32, f32), f32), line_b: ((f32, f32), f32)) -> Option<(f32, f32)> {
    let ((a_x, a_y), mut a_rot) = line_a;
    let ((b_x, b_y), mut b_rot) = line_b;

    if a_rot > PI {
        a_rot -= 2.0 * PI;
    }
    else if a_rot < -PI {
        a_rot += 2.0 * PI;
    }

    if b_rot > PI {
        b_rot -= 2.0 * PI;
    }
    else if b_rot < -PI {
        b_rot += 2.0 * PI;
    }

    let slope_a = a_rot.tan();
    let slope_b = b_rot.tan();

    let intercept_a = a_y - a_x * slope_a;
    let intercept_b = b_y - b_x * slope_b;

    if (slope_a  - slope_b).abs() <= f32::MIN {
        return None;
    }

    Some(((intercept_b - intercept_a) / (slope_a - slope_b), (slope_a * intercept_b - slope_b * intercept_a) / (slope_a - slope_b)))
}

pub fn circle_intersects_line(circle_pos: (f32, f32), circle_radius: f32, line: ((f32, f32), f32)) -> [Option<(f32, f32)>; 2] {
    let (slope, intercept) = get_slope_and_intercept(line);
    let (circle_x, circle_y) = circle_pos;

    let a = slope.powi(2) + 1.0;
    let b = 2.0 * (slope * (intercept - circle_y) - circle_x);
    let c = circle_x.powi(2) + circle_y.powi(2) + intercept.powi(2) - circle_radius.powi(2) - 2.0 * circle_y * intercept;

    let discriminante = b.powi(2) - 4.0 * a * c;

    if discriminante < 0.0 {
        return [None, None];
    }

    let root = discriminante.sqrt();

    let x_1 = (-b + root) / (2.0 * a);
    let y_1 = slope * x_1 + intercept;

    if discriminante != 0.0 {
        let x_2 = (-b - root) / (2.0 * a);
        let y_2 = slope * x_2 + intercept;
        return [Some((x_1, y_1)),
                Some((x_2, y_2))];
    }

    [Some((x_1, y_1)), None]
}

pub fn get_slope_and_intercept(line: ((f32, f32), f32)) -> (f32, f32) {
    let ((x, y), angle) = line;
    let slope = angle.tan();
    let intercept = y - x * slope;
    (slope, intercept)
}

#[test]
fn test_circle_intersects_line() {
    let circle_pos = (2.0f32, -3.0f32);
    let radius = 2.0f32;

    let slope = -1.0f32;
    let intercept = -0.5f32;

    let angle = slope.atan();
    let x = -intercept / slope;
    let y = 0.0;

    assert!(get_slope_and_intercept(((x, y), angle)) == (slope, intercept));

    let points = circle_intersects_line(circle_pos, radius, ((x, y), angle));

    assert!(points[0] != None && points[1] != None);

    let (x_1, y_1) = points[0].unwrap();
    let (x_2, y_2) = points[1].unwrap();

    assert!((round(x_1, 2) == 3.64 && round(y_1, 2) == -4.14));
    assert!((round(x_2, 2) == 0.86 && round(y_2, 2) == -1.36));

}

fn round(num: f32, precision: i32) -> f32 {
    let mult = 10.0f32.powi(precision);
    (num * mult).round() / mult
}
