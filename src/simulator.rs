use std::result::Result;
use std::cell::RefCell;
use std::string::String;
use std::f32;
use std::f32::consts::PI;

use allegro;
use allegro::{ KeyCode, TimerTick, Timer, Bitmap, Flag, Color };
use allegro_font::{ FontDrawing, FontAlign };

use allegrodata::{ AllegroData, Drawable };
use bot::Bot;

pub struct Simulator {
    allegro_data: AllegroData,
    field: Bitmap,
    bots: RefCell<Vec<Bot>>,
    field_size: (f32, f32),
    frame_pos: (f32, f32),
    frame_size: (f32, f32),
    camera_pos: (f32, f32),
    camera_view_size: (f32, f32),
    ticks: u64,
    tickrate: i32,
    timer: Timer,
}

impl Simulator {

    pub fn new(screen_size: (i32, i32), field_size: (i32, i32), tickrate: i32) -> Result<Simulator, String> {

        let allegro_data = match AllegroData::new(screen_size.0, screen_size.1) {
            Ok(e) => e,
            Err(e) => return Err(e)
        };

        let field = match Bitmap::new(allegro_data.get_core(), field_size.0, field_size.1) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could create field bitmap"))
        };

        let mut bots: Vec<Bot> = Vec::new();

        let field_size = (field_size.0 as f32, field_size.1 as f32);
        let frame_pos = (5.0, 25.0);
        let frame_size = (screen_size.0 as f32 - frame_pos.0, screen_size.1 as f32 - frame_pos.1);

        for _ in 0..20 {
            let mut bot = Bot::new(4, 4, 10.0, 5.0);
            bot.randomize_pos_rot(field_size);
            bots.push(bot);
        }

        let timer = match allegro_data.create_timer(1.0 / tickrate as f64) {
            Ok(e) => e,
            Err(e) => return Err(e)
        };

        let sim = Simulator {
            allegro_data: allegro_data,
            field: field,
            bots: RefCell::new(bots),
            field_size: field_size,
            frame_pos: frame_pos,
            frame_size: frame_size,
            camera_pos: (0.0, 0.0),
            camera_view_size: field_size,
            ticks: 0,
            tickrate: tickrate,
            timer: timer,
        };

        Ok(sim)
    }

    pub fn mainloop(&mut self) {
        let mut redraw = false;
        static TICKRATE_MOD_STEP: i32 = 10;
        static CAMERA_MOVE_STEP: f32 = 100.0;
        static CAMERA_ZOOM_FACTOR: f32 = 1.5;

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
                    KeyCode::Z => self.zoom_camera(0.75),
                    KeyCode::U => self.zoom_camera(1.25),
                    KeyCode::Left => self.move_camera((-CAMERA_MOVE_STEP, 0.0)),
                    KeyCode::Right => self.move_camera((CAMERA_MOVE_STEP, 0.0)),
                    KeyCode::Up => self.move_camera((0.0, -CAMERA_MOVE_STEP)),
                    KeyCode::Down => self.move_camera((0.0, CAMERA_MOVE_STEP)),
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

    fn move_camera(&mut self, offset: (f32, f32)) {
        self.camera_pos.0 += offset.0;
        self.camera_pos.1 += offset.1;
        match self.camera_pos.0 {
            x if x < 0.0 => self.camera_pos.0 = 0.0,
            x if x > self.field_size.0 - self.camera_view_size.0 => self.camera_pos.0 = self.field_size.0 - self.camera_view_size.0,
            _ => {}
        }
        match self.camera_pos.1 {
            y if y < 0.0 => self.camera_pos.1 = 0.0,
            y if y > self.field_size.1 - self.camera_view_size.1 => self.camera_pos.1 = self.field_size.1 - self.camera_view_size.1,
            _ => {}
        }
    }

    fn zoom_camera(&mut self, scaling: f32) {
        assert!(scaling > f32::EPSILON);
        self.camera_view_size.0 *= scaling;
        self.camera_view_size.1 *= scaling;
        if self.camera_view_size.0 > self.field_size.0 {
            self.camera_view_size.0 = self.field_size.0;
        }
        if self.camera_view_size.1 > self.field_size.1 {
            self.camera_view_size.1 = self.field_size.1;
        }
        self.move_camera((0.0, 0.0));
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

    fn point_in_view(&self, point: (f32, f32)) -> bool {
        point.0 >= self.camera_pos.0 &&
        point.1 >= self.camera_pos.1 &&
        point.0 < self.camera_pos.0 + self.camera_view_size.0 &&
        point.1 < self.camera_pos.1 + self.camera_view_size.1
    }

    fn redraw(&self) {

        let core = self.allegro_data.get_core();
        core.clear_to_color(self.allegro_data.get_black());

        core.set_target_bitmap(&self.field);
        core.clear_to_color(Color::from_rgb(22, 22, 22));

        for bot in self.bots.borrow().iter() {
            if self.point_in_view(bot.get_pos()){
                bot.draw(&self.allegro_data);
            }
        }
        self.allegro_data.get_primitives_addon().draw_rectangle(5.0, 5.0, self.field_size.0 - 5.0, self.field_size.1 - 5.0, Color::from_rgb(0xFF, 0, 0), 10.0);
        core.set_target_bitmap(self.allegro_data.get_display().get_backbuffer());

        core.draw_scaled_bitmap(&self.field, self.camera_pos.0, self.camera_pos.1, self.camera_view_size.0, self.camera_view_size.1, self.frame_pos.0, self.frame_pos.1, self.frame_size.0, self.frame_size.1, Flag::zero());

        core.draw_text(self.allegro_data.get_std_font(), self.allegro_data.get_white(), 5.0, 5.0, FontAlign::Left, &format!("bot ticks: {}", self.ticks));
        core.draw_text(self.allegro_data.get_std_font(), self.allegro_data.get_white(), 5.0, 15.0, FontAlign::Left, &format!("tickrate: {}", self.tickrate));
        core.flip_display();
    }

}
