use std::result::Result;
use std::cell::RefCell;
use std::string::String;
use std::f32;
use std::f32::consts::PI;

use bot::Bot;
use environment::Environment;

pub struct Simulator {
    bots: RefCell<Vec<Bot>>,
    min_bot_count: u32,
    field_size: (f32, f32),
    boundaries: [((f32, f32), f32); 4],
    ticks: u64,
}

impl Simulator {

    pub fn new(field_size: (i32, i32), bot_count: u32) -> Result<Simulator, String> {

        let field_size = (field_size.0 as f32, field_size.1 as f32);

        let boundaries = [  ((0.0, 0.0), 0.0),
                            (field_size, PI),
                            ((0.0, 0.0), PI / 2.0),
                            (field_size, 3.0 * PI / 2.0) ];

        let mut sim = Simulator {
            bots: RefCell::new(Vec::new()),
            min_bot_count: bot_count,
            field_size: field_size,
            boundaries: boundaries,
            ticks: 0,
        };

        sim.spawn_bots(bot_count);

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

    pub fn spawn_bot(&mut self) {
        let mut bot = Bot::new(4, 4, 10.0, 5.0);
        bot.randomize_pos_rot(self.field_size);
        self.bots.borrow_mut().push(bot);
    }

    pub fn spawn_bots(&mut self, count: u32) {
        for _ in 0..count {
            self.spawn_bot();
        }
    }

    pub fn cycle(&mut self) {
        self.ticks += 1;

        for bot in self.bots.borrow_mut().iter_mut() {
            let nearest_boundary = self.boundaries.iter()
                .map( | boundary | bot.sees_line(*boundary))
                .filter( | &e | e.is_some())
                .map(| e | e.unwrap())
                .fold((f32::MAX, 0.0), | min_val, e | match e.0 < min_val.0 { true => e, false => min_val});


            let mut environment = Environment::new(4);

            match nearest_boundary.0 < f32::MAX {
                true => {
                    environment.set_input(0, nearest_boundary.0 as f64);
                    environment.set_input(1, nearest_boundary.1 as f64);
                },
                false => {
                    environment.set_input(0, 0.0);
                    environment.set_input(1, 0.0);
                }
            }

            bot.process(environment);
        }

        self.bots.borrow_mut().retain( | ref bot | bot.in_boundary(self.field_size) && bot.get_energy() > 0);
        while self.bots.borrow().len() < self.min_bot_count as usize {
            self.spawn_bot();
        }
    }

    /*
    fn get_nearest_boundary(&self, bot: &Bot) -> (f32, f32) {




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
    }*/

}
