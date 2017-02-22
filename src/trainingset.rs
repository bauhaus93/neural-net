use std::slice::Iter;

pub struct Trainingset {
    sets: Vec<(Vec<f64>, Vec<f64>)>
}

impl Trainingset {

    pub fn new() -> Trainingset {
        Trainingset{
            sets: Vec::new()
        }
    }

    pub fn add_set(&mut self, input: Vec<f64>, output: Vec<f64>) {
        self.sets.push((input, output))
    }

    pub fn get_set_count(&self) -> usize {
        self.sets.len()
    }

    pub fn iter(&self) -> Iter<(Vec<f64>, Vec<f64>)> {
        self.sets.iter()
    }

}
