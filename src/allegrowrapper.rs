use std::result::Result;
use std::string::String;

use allegro;
use allegro_primitives;

pub struct AllegroWrapper {
    core: allegro::Core,
    display: allegro::Display,
    event_queue: allegro::EventQueue,
    timer: allegro::Timer,
    primitives: allegro_primitives::PrimitivesAddon,
    width: i32,
    height: i32,
    black: allegro::Color,
    white: allegro::Color
}

pub trait Drawable {
    fn draw(&self, allegro_wrapper: &AllegroWrapper);
}

impl AllegroWrapper {

    pub fn new(width: i32, height: i32) -> Result<AllegroWrapper, String> {
        let mut core = match allegro::Core::init() {
            Ok(e) => e,
            Err(e) => return Err(String::from("Could not init core: ") + &e)
        };

        match core.install_keyboard() {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could not install keyboard"))
        }


        let display = match allegro::Display::new(&core, width, height) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could not create display"))
        };

        let event_queue = match allegro::EventQueue::new(&core) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could not create event queue"))
        };

        let timer = match allegro::Timer::new(&core, 1.0/30.0) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could not create timer"))
        };

        let primitives = match allegro_primitives::PrimitivesAddon::init(&core) {
            Ok(e) => e,
            Err(e) => return Err(String::from("Could not init primitives: ") + &e)
        };

        event_queue.register_event_source(display.get_event_source());
        event_queue.register_event_source(core.get_keyboard_event_source());
        event_queue.register_event_source(timer.get_event_source());

        let w = display.get_width();
        let h = display.get_height();
        let black = allegro::Color::from_rgb(0, 0, 0);
        let white = allegro::Color::from_rgb(0xFF, 0xFF, 0xFF);

        let allegro_wrapper = AllegroWrapper {
            core: core,
            display: display,
            event_queue: event_queue,
            timer: timer,
            primitives: primitives,
            width: w,
            height: h,
            black: black,
            white: white
        };

        Ok(allegro_wrapper)
    }

    pub fn start_timer(&self) {
        self.timer.start();
    }

    pub fn stop_timer(&self) {
        self.timer.stop();
    }

    pub fn wait_for_event(&self) -> allegro::Event {
        self.event_queue.wait_for_event()
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn clear_black(&self) {
        self.core.clear_to_color(self.black);
    }

    pub fn flip_display(&self) {
        self.core.flip_display();
    }

    pub fn draw_filled_circle(&self, pos_x: f32, pos_y: f32, radius: f32, col: allegro::Color) {
        self.primitives.draw_filled_circle(pos_x, pos_y, radius, col);
    }

    pub fn draw_line(&self, start_x: f32, start_y: f32, stop_x: f32, stop_y: f32, col: allegro::Color, thickness: f32) {
        self.primitives.draw_line(start_x, start_y, stop_x, stop_y, col, thickness);
    }

    pub fn draw_pieslice(&self, x: f32, y: f32, radius: f32, start_theta: f32, delta_theta: f32, color: allegro::Color, thickness: f32) {
        self.primitives.draw_pieslice(x, y, radius, start_theta, delta_theta, color, thickness)
    }

    pub fn get_black(&self) -> allegro::Color {
        self.black
    }

    pub fn get_white(&self) -> allegro::Color {
        self.white
    }

}
