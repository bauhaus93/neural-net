use std::result::Result;
use std::cell::RefCell;
use std::string::String;
use std::f32;
use std::f32::consts::PI;

use allegro;

use allegrowrapper::{ AllegroWrapper, Drawable };
use bot::Bot;

pub struct Simulator {
    allegro_wrapper: AllegroWrapper,
    bots: RefCell<Vec<Bot>>,
    boundary_low: (f32, f32),
    boundary_hi: (f32, f32),
    ticks: u64,
}

impl Simulator {

    pub fn new(screen_size: (i32, i32)) -> Result<Simulator, String> {

        let wrapper = match AllegroWrapper::new(screen_size.0, screen_size.1) {
                Ok(e) => e,
                Err(e) => return Err(e)
        };

        let mut bots: Vec<Bot> = Vec::new();
        let low = (0f32, 0f32);
        let hi = (screen_size.0 as f32, screen_size.1 as f32);

        for _ in 0..20 {
            let mut bot = Bot::new(4, 4, 10.0, 5.0);
            bot.randomize_pos_rot(low, hi);
            bots.push(bot);
        }


        let sim = Simulator {
            allegro_wrapper: wrapper,
            bots: RefCell::new(bots),
            boundary_low: low,
            boundary_hi: hi,
            ticks: 0,
        };

        Ok(sim)
    }

    pub fn mainloop(&mut self) {
        let mut redraw = false;

        self.allegro_wrapper.start_timer();

        'exit: loop {


            if redraw && self.allegro_wrapper.is_event_queue_empty() {
                self.redraw();
                redraw = false;
            }

            match self.allegro_wrapper.wait_for_event() {

                allegro::KeyDown{ keycode: k, .. } =>
                    match k {
                        allegro::KeyCode::Escape => break 'exit,
                        allegro::KeyCode::F => {
                            self.allegro_wrapper.stop_timer();
                            self.fast_forward(1000);
                            self.allegro_wrapper.start_timer();
                        },
                        allegro::KeyCode::Space => self.allegro_wrapper.toggle_timer(),
                        allegro::KeyCode::I => self.allegro_wrapper.mod_timer_speed(1.25),
                        allegro::KeyCode::O => self.allegro_wrapper.mod_timer_speed(0.75),
                        _ => {}
                },


                allegro::TimerTick{..} => {
                    self.cycle_bots();
                    redraw = true;
                },

                _ => {}

            }
        }

        self.allegro_wrapper.stop_timer();
    }

    pub fn fast_forward(&mut self, cycles: u32) {
        for _ in 0..cycles {
            self.cycle_bots();
        }
    }

    fn cycle_bots(&mut self) {
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

            if !bot.in_boundary(self.boundary_low, self.boundary_hi) {
                bot.randomize_pos_rot(self.boundary_low, self.boundary_hi);
            }
        }

    }

    fn get_nearest_boundary(&self, bot: &Bot) -> (f32, f32) {

        let boundary_top = (self.boundary_low, 0.0);
        let boundary_bot = (self.boundary_hi, PI);

        let boundary_left = (self.boundary_low, PI / 2.0);
        let boundary_right = (self.boundary_hi, 3.0 * PI / 2.0);

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

    fn redraw(&self) {

        self.allegro_wrapper.clear_black();

        for bot in self.bots.borrow().iter() {
            bot.draw(&self.allegro_wrapper);

            /*let a = self.get_nearest_boundary(bot);

            if a != (0.0, 0.0) {
                println!("strike! {}, {}", a.0, a.1);
                let (dist, angle) = a;
                let (bot_x, bot_y) = bot.get_pos();
                let bot_rot = bot.get_rotation();
                let pos = (bot_x + (bot_rot + angle).cos() * dist, bot_y + (bot_rot + angle).sin() * dist);
                self.allegro_wrapper.draw_line(bot_x, bot_y, pos.0, pos.1, self.allegro_wrapper.get_white(), 4.0);
            }*/

            /*let (pos_x, pos_y) = bot.get_pos();

            for other_bot in self.bots.borrow().iter() {

                if bot.get_pos() == other_bot.get_pos() {
                    continue;
                }

                if bot.sees_point(other_bot.get_pos()) != None {
                    let (other_x, other_y) = other_bot.get_pos();
                    self.allegro_wrapper.draw_line(pos_x, pos_y, other_x, other_y, self.allegro_wrapper.get_white(), 2.0);
                }
            }*/
        }

        self.allegro_wrapper.draw_text(format!("bot ticks: {}", self.ticks), (5.0, 5.0));
        self.allegro_wrapper.draw_text(format!("tickrate: {}", self.allegro_wrapper.get_tick_rate()), (5.0, 15.0));
        self.allegro_wrapper.flip_display();
    }

}
