use std::result::Result;
use std::string::String;

use allegro::*;
use allegro_primitives::*;
use allegro_font::*;

pub struct AllegroData {
    core: Core,
    display: Display,
    event_queue: EventQueue,
    primitives_addon: PrimitivesAddon,
    font_addon: FontAddon,
    font_std: Font,
    black: Color,
    white: Color,
}

pub trait Drawable {
    fn draw(&self, allegro_data: &AllegroData);
}

impl AllegroData {

    pub fn new(width: i32, height: i32) -> Result<AllegroData, String> {
        let core = match Core::init() {
            Ok(e) => e,
            Err(e) => return Err(String::from("Could not init core: ") + &e)
        };

        //needs direct backbuffer drawing
        //core.set_new_display_option(DisplayOption::SampleBuffers, 1, DisplayOptionImportance::Require);
        //core.set_new_display_option(DisplayOption::Samples, 8, DisplayOptionImportance::Suggest);

        let display = match Display::new(&core, width, height) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could not create display"))
        };

        let event_queue = match EventQueue::new(&core) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could not create event queue"))
        };

        let primitives_addon = match PrimitivesAddon::init(&core) {
            Ok(e) => e,
            Err(e) => return Err(String::from("Could not init primitives addon: ") + &e)
        };

        let font_addon = match FontAddon::init(&core) {
            Ok(e) => e,
            Err(e) => return Err(String::from("Could not init font addon: ") + &e)
        };

        let font = match Font::new_builtin(&font_addon) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could not load builtin font"))
        };

        match core.install_keyboard() {
            Ok(_) => {},
            Err(_) => return Err(String::from("Could not install keyboard"))
        }

        match core.install_mouse() {
            Ok(_) => {},
            Err(_) => return Err(String::from("Could not install mouse"))
        }

        event_queue.register_event_source(display.get_event_source());

        match core.get_keyboard_event_source() {
            Some(e) => event_queue.register_event_source(e),
            None => return Err(String::from("Could not retrieve keyboard event source"))
        }

        match core.get_mouse_event_source() {
            Some(e) => event_queue.register_event_source(e),
            None => return Err(String::from("Could not retrieve mouse event source"))
        }


        let allegro_data = AllegroData {
            core: core,
            display: display,
            event_queue: event_queue,
            primitives_addon: primitives_addon,
            font_addon: font_addon,
            font_std: font,
            black: Color::from_rgb(0, 0, 0),
            white: Color::from_rgb(0xFF, 0xFF, 0xFF)
        };

        Ok(allegro_data)
    }

    pub fn get_core(&self) -> &Core {
        &self.core
    }

    pub fn get_display(&self) -> &Display {
        &self.display
    }

    pub fn get_event_queue(&self) -> &EventQueue {
        &self.event_queue
    }

    pub fn get_primitives_addon(&self) -> &PrimitivesAddon {
        &self.primitives_addon
    }

    pub fn get_font_addon(&self) -> &FontAddon {
        &self.font_addon
    }

    pub fn get_std_font(&self) -> &Font {
        &self.font_std
    }

    pub fn get_black(&self) -> Color {
        self.black
    }

    pub fn get_white(&self) -> Color {
        self.white
    }

    pub fn create_timer(&self, speed: f64) -> Result<Timer, String> {
        let timer = match Timer::new(&self.core, speed) {
            Ok(e) => e,
            Err(_) => return Err(String::from("Could not create timer"))
        };

        self.event_queue.register_event_source(timer.get_event_source());

        Ok(timer)
    }

    /*pub fn is_event_queue_empty(&self) -> bool {
        self.event_queue.is_empty()
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

    pub fn set_window_title(&self, new_title: String) {
        self.display.set_window_title(&new_title);
    }

    pub fn draw_text(&self, text: String, position: (f32, f32)) {
        self.core.draw_text(&self.font, self.white, position.0, position.1, FontAlign::Left, &text);
    }

    pub fn draw_filled_circle(&self, pos_x: f32, pos_y: f32, radius: f32, col: allegro::Color) {
        self.primitives_addon.draw_filled_circle(pos_x, pos_y, radius, col);
    }

    pub fn draw_line(&self, start_x: f32, start_y: f32, stop_x: f32, stop_y: f32, col: allegro::Color, thickness: f32) {
        self.primitives_addon.draw_line(start_x, start_y, stop_x, stop_y, col, thickness);
    }

    pub fn draw_pieslice(&self, x: f32, y: f32, radius: f32, start_theta: f32, delta_theta: f32, color: allegro::Color, thickness: f32) {
        self.primitives_addon.draw_pieslice(x, y, radius, start_theta, delta_theta, color, thickness)
    }

    pub fn get_black(&self) -> allegro::Color {
        self.black
    }

    pub fn get_white(&self) -> allegro::Color {
        self.white
    }*/

}
