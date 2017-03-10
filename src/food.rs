use allegro::Color;


use allegrodata::{ AllegroData, Drawable };

pub struct Food {
    pos: (f32, f32),
    size: f32,
    energy: u32,
    color: Color
}

impl Drawable for Food {
    fn draw(&self, allegro_data: &AllegroData, camera_pos: (f32, f32), scale: (f32, f32)) {

        allegro_data.get_primitives_addon().draw_filled_circle(
            (self.pos.0 - camera_pos.0) * scale.0,
            (self.pos.1 - camera_pos.1) * scale.1,
            self.size * scale.0,
            self.color);
    }
}

impl Food {

    pub fn new(pos: (f32, f32), size: f32, energy: u32) -> Food {
        Food {
            pos: pos,
            size: size,
            energy: energy,
            color: Color::from_rgb(0x70, 0x20, 0xF)
        }
    }

    pub fn get_energy(&self) -> u32 {
        self.energy
    }

    pub fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    pub fn get_size(&self) -> f32 {
        self.size
    }

}
