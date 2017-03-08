use std::result::Result;
use std::cell::RefCell;
use std::string::String;
use std::f32;
use std::f32::consts::PI;

use bot::Bot;

pub struct Simulator {
    bots: RefCell<Vec<Bot>>,
    field_size: (f32, f32),
    ticks: u64,
}

impl Simulator {

    pub fn new(field_size: (i32, i32), bot_count: u32) -> Result<Simulator, String> {

        let mut bots: Vec<Bot> = Vec::new();

        let field_size = (field_size.0 as f32, field_size.1 as f32);

        for _ in 0..bot_count {
            let mut bot = Bot::new(4, 4, 10.0, 5.0);
            bot.randomize_pos_rot(field_size);
            bots.push(bot);
        }

        let sim = Simulator {
            bots: RefCell::new(bots),
            field_size: field_size,
            ticks: 0,
        };

        Ok(sim)
    }

    pub fn get_bots(&self) -> &RefCell<Vec<Bot>> {
        &self.bots
    }

    pub fn get_field_size(&self) -> (f32, f32) {
        self.field_size
    }

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub fn fast_forward(&mut self, cycles: u32) {
        for _ in 0..cycles {
            self.cycle();
        }
    }

    pub fn cycle(&mut self) {
        self.ticks += 1;
        for bot in self.bots.borrow_mut().iter_mut() {

            let boundary = self.get_nearest_boundary(bot);

            let env = vec![boundary.0 as f64, boundary.1 as f64, 0.0, 0.0];

            bot.process(&env);

            let mut feedback = vec![0f64; 4];
            if env[0] > 0.0 {
                if env[1] > 0.0 {
                    if (env[1] as f32) < PI {
                        feedback[1] = 1.0;
                    }
                }
                else if env[1] <= 0.0 {
                    if (env[1] as f32) > -PI {
                        feedback[0] = 1.0;
                    }
                }
            }

            bot.give_feedback(&feedback);

            if !bot.in_boundary(self.field_size) {
                bot.randomize_pos_rot(self.field_size);
            }
        }

    }

    fn get_nearest_boundary(&self, bot: &Bot) -> (f32, f32) {

        //TODO save somewhere in class
        let boundary_top = ((0.0, 0.0), 0.0);
        let boundary_bot = (self.field_size, PI);

        let boundary_left = ((0.0, 0.0), PI / 2.0);
        let boundary_right = (self.field_size, 3.0 * PI / 2.0);

        let boundaries = [  bot.sees_line(boundary_top),
                            bot.sees_line(boundary_bot),
                            bot.sees_line(boundary_left),
                            bot.sees_line(boundary_right) ];

        let mut nearest = (f32::MAX, 0.0);

        for boundary in boundaries.iter() {
            if *boundary != None {
                let boundary = boundary.unwrap();
                if boundary.0 < nearest.0 {
                    nearest = boundary;
                }
            }
        }

        if nearest.0 == f32::MAX {
            return (0.0, 0.0);
        }
        nearest
    }

}
