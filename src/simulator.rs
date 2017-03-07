use std::result::Result;
use std::cell::RefCell;
use std::string::String;
use std::f32;
use std::f32::consts::PI;

use allegro;
use allegro::{ KeyCode, TimerTick, Timer };
use allegro_font::{ FontDrawing, FontAlign };

use allegrodata::{ AllegroData, Drawable };
use bot::Bot;

pub struct Simulator {
    allegro_data: AllegroData,
    bots: RefCell<Vec<Bot>>,
    boundary_low: (f32, f32),
    boundary_hi: (f32, f32),
    ticks: u64,
    tickrate: i32,
    timer: Timer,
}

impl Simulator {

    pub fn new(screen_size: (i32, i32), tickrate: i32) -> Result<Simulator, String> {

        let allegro_data = match AllegroData::new(screen_size.0, screen_size.1) {
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

        let timer = match allegro_data.create_timer(1.0 / tickrate as f64) {
            Ok(e) => e,
            Err(e) => return Err(e)
        };

        let sim = Simulator {
            allegro_data: allegro_data,
            bots: RefCell::new(bots),
            boundary_low: low,
            boundary_hi: hi,
            ticks: 0,
            tickrate: tickrate,
            timer: timer,
        };

        Ok(sim)
    }

    pub fn mainloop(&mut self) {
        let mut redraw = false;
        static TICKRATE_MOD_STEP: i32 = 10;

        self.timer.start();

        'exit: loop {


            if redraw && self.allegro_data.get_event_queue().is_empty() {
                self.redraw();
                redraw = false;
            }

            match self.allegro_data.get_event_queue().wait_for_event() {

                allegro::KeyDown{ keycode: k, .. } => match k {
                    KeyCode::Escape => break 'exit,
                    KeyCode::F => {
                        self.timer.stop();
                        self.fast_forward(1000);
                        self.timer.start();
                    },
                    KeyCode::Space => self.toggle_timer(),
                    KeyCode::I => self.mod_speed(TICKRATE_MOD_STEP),
                    KeyCode::O => self.mod_speed(-TICKRATE_MOD_STEP),
                    _ => {}
                },

                TimerTick{..} => {
                    self.cycle_bots();
                    redraw = true;
                },
                _ => {}
            }
        }

        self.timer.stop();
    }

    fn toggle_timer(&self) {
        match self.timer.is_started() {
            true => self.timer.stop(),
            false => self.timer.start()
        }
    }

    fn mod_speed(&mut self, tickrate_mod: i32) {
        self.tickrate += tickrate_mod;
        match self.tickrate {
            tr_mod if tr_mod < 10 => self.tickrate = 10,
            tr_mod if tr_mod > 500 => self.tickrate = 500,
            _ => {}
        }
        self.timer.set_speed(1.0 / self.tickrate as f64)
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

        let core = self.allegro_data.get_core();

        core.clear_to_color(self.allegro_data.get_black());

        for bot in self.bots.borrow().iter() {
            bot.draw(&self.allegro_data);
        }

        core.draw_text(self.allegro_data.get_std_font(), self.allegro_data.get_white(), 5.0, 5.0, FontAlign::Left, &format!("bot ticks: {}", self.ticks));
        core.draw_text(self.allegro_data.get_std_font(), self.allegro_data.get_white(), 5.0, 15.0, FontAlign::Left, &format!("tickrate: {}", self.tickrate));
        core.flip_display();
    }

}
