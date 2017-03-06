
pub struct Ray{
    origin: (f32, f32),
    rotation: f32
}

impl Ray {

    pub fn new(origin: (f32, f32), rotation: f32) -> Ray {
        Ray {
            origin: origin,
            rotation: rotation
        }
    }

}
