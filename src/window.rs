use std::option::Option;
use std::f32;
use std::time::{ Duration, Instant };

use allegro::{ KeyCode, KeyDown, MouseButtonDown, MouseAxes, TimerTick, Timer, Bitmap, Flag, Color };
use allegro_font::{ FontDrawing, FontAlign };

use allegrodata::{ AllegroData, Drawable };
use simulator::Simulator;

pub struct Window {
    allegro_data: AllegroData,
    simulator: Simulator,
    frame_pos: (f32, f32),
    frame_size: (f32, f32),
    camera_pos: (f32, f32),
    camera_view_size: (f32, f32),
    scale: (f32, f32),
    field_bmp: Bitmap,
    tickrate: i32,
    timer_bot_update: Timer,
    timer_redraw: Timer,
}

pub struct WindowBuilder {
    screen_size: (i32, i32),
    frame_pos: (f32, f32),
    frame_size: (f32, f32),
    tickrate: i32,
    redraw_rate: u32,
    simulator: Option<Simulator>
}

impl WindowBuilder {

    pub fn new(screen_size: (i32, i32)) -> WindowBuilder {
        assert!(screen_size.0 > 0);
        assert!(screen_size.1 > 0);

        WindowBuilder {
            screen_size: screen_size,
            frame_pos: (0.0, 0.0),
            frame_size: (screen_size.0 as f32, screen_size.1 as f32),
            tickrate: 30,
            redraw_rate: 30,
            simulator: None
        }
    }

    pub fn frame_pos(mut self, frame_pos: (f32, f32)) -> Self {
        self.frame_pos = frame_pos;
        self
    }

    pub fn frame_size(mut self, frame_size: (f32, f32)) -> Self {
        self.frame_size = frame_size;
        self
    }

    pub fn tickrate(mut self, tickrate: i32) -> Self {
        self.tickrate = tickrate;
        self
    }

    pub fn redraw_rate(mut self, redraw_rate: u32) -> Self {
        self.redraw_rate = redraw_rate;
        self
    }

    pub fn simulator(mut self, sim: Simulator) -> Self {
        self.simulator = Some(sim);
        self
    }

    pub fn finish(mut self) -> Result<Window, String> {

        let allegro_data = match AllegroData::new(self.screen_size.0 as i32, self.screen_size.1 as i32) {
            Ok(e) => e,
            Err(e) => return Err(e)
        };

        let field_bmp = match Bitmap::new(allegro_data.get_core(), self.frame_size.0 as i32, self.frame_size.1 as i32) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could create field bitmap"))
        };

        let timer_tick = match allegro_data.create_timer(1.0 / self.tickrate as f64) {
            Ok(e) => e,
            Err(e) => return Err(e)
        };

        let timer_redraw = match allegro_data.create_timer(1.0 / self.redraw_rate as f64) {
            Ok(e) => e,
            Err(e) => return Err(e)
        };

        if self.simulator.is_none() {
            self.simulator = match Simulator::new((self.frame_size.0 as i32, self.frame_size.1 as i32), 20) {
                Ok(sim) => Some(sim),
                Err(e) => return Err(e)
            };
        }

        let simulator = self.simulator.unwrap();
        let field_size = simulator.get_field_size();

        let window = Window {
            allegro_data: allegro_data,
            simulator: simulator,
            frame_pos: self.frame_pos,
            frame_size: self.frame_size,
            camera_pos: (0.0, 0.0),
            camera_view_size: field_size,
            scale: (self.frame_size.0 / field_size.0, self.frame_size.1 / field_size.1),
            field_bmp: field_bmp,
            tickrate: self.tickrate,
            timer_bot_update: timer_tick,
            timer_redraw: timer_redraw
        };

        Ok(window)
    }
}


impl Window {

    pub fn mainloop(&mut self) {
        let mut redraw = false;
        static TICKRATE_MOD_STEP: i32 = 10;
        static CAMERA_MOVE_STEP: f32 = 100.0;
        static CAMERA_ZOOM_FACTOR: f32 = 1.5;

        self.timer_bot_update.start();
        self.timer_redraw.start();

        'exit: loop {

            if redraw && self.allegro_data.get_event_queue().is_empty() {
                self.redraw();
                redraw = false;
            }

            match self.allegro_data.get_event_queue().wait_for_event() {

                KeyDown{ keycode: k, .. } => match k {
                    KeyCode::Escape => break 'exit,
                    KeyCode::F => {
                        self.timer_bot_update.stop();
                        self.simulator.fast_forward(1000);
                        self.timer_bot_update.start();
                    },
                    KeyCode::Space => self.toggle_timers(),
                    KeyCode::I => self.mod_speed(TICKRATE_MOD_STEP),
                    KeyCode::O => self.mod_speed(-TICKRATE_MOD_STEP),
                    KeyCode::Left => self.move_camera((-CAMERA_MOVE_STEP, 0.0)),
                    KeyCode::Right => self.move_camera((CAMERA_MOVE_STEP, 0.0)),
                    KeyCode::Up => self.move_camera((0.0, -CAMERA_MOVE_STEP)),
                    KeyCode::Down => self.move_camera((0.0, CAMERA_MOVE_STEP)),
                    _ => {}
                },

                MouseButtonDown{ button: b, x: pos_x, y: pos_y, .. } => match b {
                    //is also checked in mouse_pos_to_frame_pos
                    //but here to not catch all left clicks (even those outside the field frame)
                    1 if self.point_in_frame((pos_x as f32, pos_y as f32))  => {
                        if let Some(frame_pos) = self.mouse_pos_to_frame_pos((pos_x as f32, pos_y as f32)) {
                            self.move_camera_to_mouse_click(frame_pos);
                        }

                        },
                    _ => {}
                },

                //Currently a little bit rough in it's camera movement
                MouseAxes{ x: pos_x, y: pos_y, dz: wheel_rotation, .. } => {
                    if let Some(click_pos) = self.mouse_pos_to_frame_pos((pos_x as f32, pos_y as f32)) {
                        match wheel_rotation {
                            1 => self.zoom_and_position_camera(0.9, click_pos),
                            -1 => self.zoom_and_position_camera(1.1, click_pos),
                            _ => {}
                        }
                    }
                },

                ///Unrough version, without camera movement
                /*MouseAxes{ dz: wheel_rotation, .. } => match wheel_rotation {
                    1 => self.zoom_camera(0.9),
                    -1 => self.zoom_camera(1.1),
                    _ => {}
                },*/

                TimerTick{source: src, ..} => {
                    match src == self.timer_redraw.get_event_source().get_event_source() {
                        true => redraw = true,
                        false => self.simulator.cycle()
                    }
                },
                _ => {}
            }
        }

        self.timer_bot_update.stop();
        self.timer_redraw.stop();
    }

    fn move_camera(&mut self, offset: (f32, f32)) {
        self.camera_pos.0 += offset.0;
        self.camera_pos.1 += offset.1;
        self.check_camera_pos();
    }

    fn move_camera_to_mouse_click(&mut self, click_pos: (f32, f32)) {
        self.camera_pos.0 = self.camera_pos.0 + self.camera_view_size.0 * (click_pos.0 / self.frame_size.0) - self.camera_view_size.0 / 2.0;
        self.camera_pos.1 = self.camera_pos.1 + self.camera_view_size.1 * (click_pos.1 / self.frame_size.1) - self.camera_view_size.1 / 2.0;
        self.check_camera_pos();
    }

    fn check_camera_pos(&mut self) {
        let field_size = self.simulator.get_field_size();
        match self.camera_pos.0 {
            x if x < 0.0 => self.camera_pos.0 = 0.0,
            x if x > field_size.0 - self.camera_view_size.0 => self.camera_pos.0 = field_size.0 - self.camera_view_size.0,
            _ => {}
        }
        match self.camera_pos.1 {
            y if y < 0.0 => self.camera_pos.1 = 0.0,
            y if y > field_size.1 - self.camera_view_size.1 => self.camera_pos.1 = field_size.1 - self.camera_view_size.1,
            _ => {}
        }
    }

    fn zoom_and_position_camera(&mut self, scaling: f32, target_pos: (f32, f32)) {
        self.zoom_camera(scaling);
        if scaling < 1.0 {
            self.move_camera_to_mouse_click(target_pos);
        }
    }

    fn zoom_camera(&mut self, scaling: f32) {
        assert!(scaling > f32::EPSILON);
        let field_size = self.simulator.get_field_size();

        self.camera_view_size.0 *= scaling;
        self.camera_view_size.1 *= scaling;

        if self.camera_view_size.0 > field_size.0 {
            self.camera_view_size.0 = field_size.0;
            self.camera_view_size.1 = field_size.1;
        }

        self.scale.0 = self.frame_size.0 / self.camera_view_size.0;
        self.scale.1 = self.frame_size.1 / self.camera_view_size.1;

        self.move_camera((0.0, 0.0));
    }

    fn toggle_timers(&self) {
        match self.timer_bot_update.is_started() {
            true => self.timer_bot_update.stop(),
            false => self.timer_bot_update.start()
        }

        match self.timer_redraw.is_started() {
            true => self.timer_redraw.stop(),
            false => self.timer_redraw.start()
        }
    }

    fn mod_speed(&mut self, tickrate_mod: i32) {
        self.tickrate += tickrate_mod;
        match self.tickrate {
            tr_mod if tr_mod < 10 => self.tickrate = 10,
            tr_mod if tr_mod > 500 => self.tickrate = 500,
            _ => self.timer_bot_update.set_speed(1.0 / self.tickrate as f64)
        }
    }

    fn point_in_view(&self, point: (f32, f32)) -> bool {
        point.0 >= self.camera_pos.0 &&
        point.1 >= self.camera_pos.1 &&
        (point.0 - self.camera_pos.0) * self.scale.0 < self.frame_size.0 &&
        (point.1 - self.camera_pos.1) * self.scale.1 < self.frame_size.1
    }

    fn point_in_frame(&self, point: (f32, f32)) -> bool {
        point.0 >= self.frame_pos.0 &&
        point.1 >= self.frame_pos.1 &&
        point.0 < self.frame_pos.0 + self.frame_size.0 &&
        point.1 < self.frame_pos.1 + self.frame_size.1
    }

    fn mouse_pos_to_frame_pos(&self, click_pos: (f32, f32)) -> Option<(f32, f32)> {
        match self.point_in_frame(click_pos) {
            true => Some((click_pos.0 - self.frame_pos.0, click_pos.1 - self.frame_pos.1)),
            false => None
        }
    }

    /*Redrawing with frequency of 60+ suddenly raised the execution time of
    core.set_target_bitmap() AND
    core.draw_text, if set_target_bitmap was disabled
    Maybe related to max display frequency? But until now, there weren't any
    CPU spikes if redraw frequency was higher than 60
    */
    fn redraw(&self) {
        /*static mut c: u32 = 0;
        let now = Instant::now();*/

        const BORDER_THICKNESS: f32 = 2.0;
        const BORDER_THICKNESS_HALF: f32 = BORDER_THICKNESS / 2.0;

        let core = self.allegro_data.get_core();
        let field_size = self.simulator.get_field_size();

        core.clear_to_color(self.allegro_data.get_black());


        core.set_target_bitmap(&self.field_bmp);

        core.clear_to_color(Color::from_rgb(22, 22, 22));

        for bot in self.simulator.get_bots().borrow().iter() {
            if self.point_in_view(bot.get_pos()){
                bot.draw(&self.allegro_data, self.camera_pos, self.scale);
            }
        }

        if self.camera_pos.0 < f32::EPSILON {
            self.allegro_data.get_primitives_addon().draw_line(BORDER_THICKNESS_HALF, 0.0, BORDER_THICKNESS_HALF, self.frame_size.1, Color::from_rgb(0xFF, 0, 0), BORDER_THICKNESS);
        }

        if self.camera_pos.0 + self.camera_view_size.0 >= field_size.0 {
            self.allegro_data.get_primitives_addon().draw_line(self.frame_size.0 - BORDER_THICKNESS_HALF, 0.0, self.frame_size.0 - BORDER_THICKNESS_HALF, self.frame_size.1, Color::from_rgb(0xFF, 0, 0), BORDER_THICKNESS);
        }

        if self.camera_pos.1 < f32::EPSILON {
            self.allegro_data.get_primitives_addon().draw_line(0.0, BORDER_THICKNESS_HALF, self.frame_size.0, BORDER_THICKNESS_HALF, Color::from_rgb(0xFF, 0, 0), BORDER_THICKNESS);
        }

        if self.camera_pos.1 + self.camera_view_size.1 >= field_size.1 {
            self.allegro_data.get_primitives_addon().draw_line(0.0, self.frame_size.1 - BORDER_THICKNESS_HALF, self.frame_size.0, self.frame_size.1 - BORDER_THICKNESS_HALF, Color::from_rgb(0xFF, 0, 0), BORDER_THICKNESS);
        }

        core.set_target_bitmap(self.allegro_data.get_display().get_backbuffer());

        core.draw_bitmap(&self.field_bmp, self.frame_pos.0, self.frame_pos.1, Flag::zero());

        core.draw_text(self.allegro_data.get_std_font(), self.allegro_data.get_white(), 5.0, 5.0, FontAlign::Left, &format!("bot ticks: {}", self.simulator.get_ticks()));
        core.draw_text(self.allegro_data.get_std_font(), self.allegro_data.get_white(), 5.0, 15.0, FontAlign::Left, &format!("tickrate: {}", self.tickrate));


        core.flip_display();

        /*unsafe{
            if c % 30 == 0 {
                println!("time: {:?} us", (now.elapsed().subsec_nanos() / 1000));
            }
            c += 1;
        }*/

    }

}
