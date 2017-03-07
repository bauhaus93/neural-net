use std::f32;
use std::f32::consts::PI;
use std::option::Option;

use rand::distributions::{ Range, IndependentSample };
use rand;
use allegro;

use neuralnet::NeuralNet;
use allegrodata::{ AllegroData, Drawable };
use utility::{ get_distance, line_intersects_line, Vector2D };


pub struct Bot {
    nn: NeuralNet,
    pos: (f32, f32),
    rot: f32,
    size: f32,
    speed: f32,
    view_radius: f32,
    fov: f32,
    color: allegro::Color
}

pub enum Direction {
    Left,
    Right
}

impl Drawable for Bot {
    fn draw(&self, allegro_data: &AllegroData) {
        //let x = self.pos.0 + self.view_radius * f32::cos(self.rot);
        //let y = self.pos.1 + self.view_radius * f32::sin(self.rot);
        //allegro_wrapper.draw_line(self.pos.0, self.pos.1, x, y, allegro_wrapper.get_white(), 1.0);

        allegro_data.get_primitives_addon().draw_pieslice(self.pos.0, self.pos.1,
                                      self.view_radius,
                                      self.rot - self.fov / 2.0,
                                      self.fov,
                                      allegro_data.get_white(),
                                      1.0);

        allegro_data.get_primitives_addon().draw_filled_circle(self.pos.0, self.pos.1, self.size, self.color);
    }
}

impl Bot {

    pub fn new(layers: usize, units: usize, size: f32, speed: f32) -> Bot {
        let mut nn = NeuralNet::new(layers, units);

        nn.randomize(-1.0, 1.0);

        let mut bot = Bot {
            nn: nn,
            pos: (0.0, 0.0),
            rot: 0.0,
            size: size,
            speed: speed,
            view_radius: 8.0 * size,
            fov: PI / 2.0,
            color: allegro::Color::from_rgb(0xFF, 0xFF, 0xFF)
        };

        bot.randomize_color();

        bot
    }

    pub fn process(&mut self, environment: &Vec<f64>) -> Vec<f64> {

        let actions = self.nn.feed_forward(environment);

        if actions[0] > 0.75 && actions[0] > actions[1] {
            self.rotate(Direction::Left);
        }
        else if actions[1] > 0.75 && actions[1] > actions[0] {
            self.rotate(Direction::Right);
        }

        self.shift();

        actions
    }

    pub fn give_feedback(&mut self, feedback: &Vec<f64>) {
        self.nn.backpropagate(feedback, 1.0);
    }

    pub fn randomize_net(&mut self, min: f64, max: f64) {
        self.nn.randomize(min, max);
    }

    pub fn randomize_pos_rot(&mut self, boundary_low: (f32, f32), boundary_hi: (f32, f32)) {
        let range_x = Range::new(boundary_low.0, boundary_hi.0);
        let range_y = Range::new(boundary_low.1, boundary_hi.1);
        let range_rot = Range::new(0.0, 2.0 * f32::consts::PI);
        let mut rng = rand::thread_rng();

        self.pos = (range_x.ind_sample(&mut rng), range_y.ind_sample(&mut rng));
        self.rot = range_rot.ind_sample(&mut rng);
    }

    pub fn randomize_color(&mut self) {
        let range = Range::new(0, 0xFF);
        let mut rng = rand::thread_rng();

        let r = range.ind_sample(&mut rng);
        let g = range.ind_sample(&mut rng);
        let b = range.ind_sample(&mut rng);
        self.color = allegro::Color::from_rgb(r, g, b);
    }

    fn shift(&mut self) {
        self.pos.0 += self.speed * f32::cos(self.rot);
        self.pos.1 += self.speed * f32::sin(self.rot);
    }

    pub fn rotate(&mut self, dir: Direction) {
        let offset = f32::consts::PI / 15.0;

        match dir {
            Direction::Left => self.rot += offset,
            Direction::Right => self.rot -= offset
        }
    }

    pub fn in_boundary(&self, low: (f32, f32), hi: (f32, f32)) -> bool {
        assert!(low.0 < hi.0);
        assert!(low.0 < hi.1);

        if self.pos.0 < low.0 || self.pos.0 > hi.0 || self.pos.1 < low.1 || self.pos.1 > hi.1 {
            return false;
        }
        true
    }

    pub fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    pub fn set_pos(&mut self, new_pos: (f32, f32)) {
        self.pos = new_pos;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rot
    }

    pub fn get_view_radius(&self) -> f32 {
        self.view_radius
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    fn get_view_vector(&self) -> (f32, f32) {
        (self.view_radius * self.rot.cos(),
         self.view_radius * self.rot.sin())
    }

    fn get_rotated_view_vector(&self, angle_offset: f32) -> (f32, f32) {
        (self.view_radius * (self.rot + angle_offset).cos(),
         self.view_radius * (self.rot + angle_offset).sin())
    }

    pub fn sees_point(&self, point: (f32, f32)) -> Option<(f32, f32)> {

        let distance = get_distance(self.pos, point);

        if distance < self.view_radius {
            let view = self.get_view_vector();
            let target = (point.0 - self.pos.0, point.1 - self.pos.1);

            let angle = view.get_angle_diff(target);

            let fov_half = self.fov / 2.0;

            if angle >= -fov_half && angle <= fov_half {
                return Some((distance, angle));
            }
        }

        None
    }

    pub fn sees_line(&self, line: ((f32, f32), f32)) -> Option<(f32, f32)> {

        let point = self.get_nearest_point_on_line(line);
        let shortest_distance = get_distance(self.pos, point);

        if shortest_distance < self.view_radius {
            let angle = self.get_view_vector().get_angle_diff(point.sub(self.pos));
            let fov_half = self.fov / 2.0;

            if angle.abs() < fov_half {
                return Some((shortest_distance, angle));
            }
            else{
                let left_angle = self.rot - fov_half;
                let right_angle = self.rot + fov_half;

                let left_ray = (self.pos, left_angle);
                let right_ray = (self.pos, right_angle);

                let left_intersection = line_intersects_line(left_ray, line);
                let right_intersection = line_intersects_line(right_ray, line);


                if left_intersection != None && right_intersection != None {

                    let left_intersection = left_intersection.unwrap();
                    let right_intersection = right_intersection.unwrap();

                    //direction can be +1.0 or -1.0, depending on if the intersection is in view or 180° behind it
                    let left_direction = self.get_rotated_view_vector(-fov_half).normalize().dot(left_intersection.sub(self.pos).normalize());
                    let right_direction = self.get_rotated_view_vector(fov_half).normalize().dot(right_intersection.sub(self.pos).normalize());

                    if left_direction > 0.0 && right_direction > 0.0 {
                        let left_distance = get_distance(self.pos, left_intersection);
                        let right_distance = get_distance(self.pos, right_intersection);

                        if left_distance < right_distance && left_distance < self.view_radius {
                            return Some((left_distance, -fov_half));
                        }
                        else if right_distance < self.view_radius {
                            return Some((right_distance, fov_half));
                        }
                    }
                    else if left_direction > 0.0 {
                        let left_distance = get_distance(self.pos, left_intersection);
                        if left_distance < self.view_radius {
                            return Some((left_distance, -fov_half));
                        }
                    }
                    else if right_direction > 0.0 {
                        let right_distance = get_distance(self.pos, right_intersection);
                        if right_distance < self.view_radius {
                            return Some((right_distance, fov_half));
                        }
                    }
                }

            }
        }
        None
    }

    fn get_nearest_point_on_line(&self, line: ((f32, f32), f32)) -> (f32, f32) {
        line_intersects_line((self.pos, line.1 + PI / 2.0), line).unwrap()
    }

}
