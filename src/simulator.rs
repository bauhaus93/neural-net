use std::result::Result;
use std::cell::RefCell;
use std::string::String;
use std::f32;

use allegro;
use allegro_primitives;

use allegrowrapper::{ AllegroWrapper, Drawable };
use bot::Bot;

const PI_DOUBLE: f32 = f32::consts::PI * 2.0;
const PI_HALF: f32 = f32::consts::PI / 2.0;

pub struct Simulator {
    allegro_wrapper: AllegroWrapper,
    bots: RefCell<Vec<Bot>>,
    boundary_low: (f32, f32),
    boundary_hi: (f32, f32),
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
            boundary_hi: hi
        };

        Ok(sim)
    }

    pub fn mainloop(&self) {
        let mut redraw = false;

        self.allegro_wrapper.start_timer();

        'exit: loop {

            if redraw {
                self.redraw();
                redraw = false;
            }

            match self.allegro_wrapper.wait_for_event() {

                allegro::KeyDown{keycode: k, ..} =>
                    match k {
                        allegro::KeyCode::Escape => break 'exit,
                        allegro::KeyCode::F => self.fast_forward(1000),
                        _ => println!("Uncaught Keydown")
                },

                allegro::TimerTick{..} => {
                    self.cycle_bots();
                    redraw = true;
                },

                _ => println!("Uncaught event")

            }
        }

        self.allegro_wrapper.stop_timer();
    }

    pub fn fast_forward(&self, cycles: u32) {
        for _ in 0..cycles {
            self.cycle_bots();
        }
    }

    fn cycle_bots(&self) {

        for bot in self.bots.borrow_mut().iter_mut() {

            let boundary = self.get_nearest_boundary(bot);

            let env = vec![boundary.0 as f64, boundary.1 as f64, 0.0, 0.0];

            bot.process(&env);

            let mut feedback = vec![0f64; 4];
            if env[0] > 0.0 {
                if env[1] > 0.0 {
                    feedback[1] = 1.0;
                }
                else {
                    feedback[0] = 1.0;
                }
            }

            bot.give_feedback(&feedback);

            if !bot.in_boundary(self.boundary_low, self.boundary_hi) {
                bot.randomize_pos_rot(self.boundary_low, self.boundary_hi);
            }
        }

    }

    fn get_nearest_boundary(&self, bot: &Bot) -> (f32, f32) {
        let rot = bot.get_rotation();
        let pos = bot.get_pos();
        let view_radius = bot.get_view_radius();
        let fov = bot.get_fov();

        //left boundary
        let mut min_dist = get_distance(pos, (0.0, pos.1));
        let mut angle = f32::consts::PI - rot;

        //top boundary
        let dist = get_distance(pos, (pos.0, 0.0));
        if dist < min_dist {
            min_dist = dist;
            angle = 3.0 * PI_HALF - rot;
        }

        //right boundary
        let dist = get_distance(pos, (self.boundary_hi.0, pos.1));
        if dist < min_dist {
            min_dist = dist;
            angle = 0.0 - rot;
        }

        //bottom boundary
        let dist = get_distance(pos, (pos.0, self.boundary_hi.1));
        if dist < min_dist {
            min_dist = dist;
            angle = PI_HALF - rot;
        }

        if min_dist < view_radius && angle.abs() < PI_HALF {
            return (min_dist, angle);
        }
        (0.0, 0.0)
    }

    fn redraw(&self) {

        self.allegro_wrapper.clear_black();

        for bot in self.bots.borrow().iter() {
            bot.draw(&self.allegro_wrapper);

            let (pos_x, pos_y) = bot.get_pos();

            for other_bot in self.bots.borrow().iter() {

                if bot.get_pos() == other_bot.get_pos() {
                    continue;
                }

                if bot.sees(other_bot.get_pos()) {
                    let (other_x, other_y) = other_bot.get_pos();
                    self.allegro_wrapper.draw_line(pos_x, pos_y, other_x, other_y, self.allegro_wrapper.get_white(), 2.0);
                }
            }
        }

        self.allegro_wrapper.flip_display();
    }
    
}

fn get_distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dist_x = a.0 - b.0;
    let dist_y = a.1 - b.1;
    f32::sqrt(dist_x * dist_x + dist_y * dist_y)
}
