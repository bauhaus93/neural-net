use std::f64::consts::PI;

pub struct Environment {
    input: Vec<f64>,
}

impl Environment {

    pub fn new(size: usize) -> Environment {
        Environment {
            input: vec![0.0; size],
        }
    }

    pub fn set_input(&mut self, index: usize, value: f64) {
        self.input[index] = value;
    }

    pub fn get_input(&self) -> &Vec<f64> {
        &self.input
    }

    #[allow(unused_variables)]
    pub fn get_expected_output(&self, output: &Vec<f64>) -> Vec<f64> {
        let mut target_output = vec![0.0; self.input.len()];

        if self.input[0] > 0.0 {
            match self.input[1] {
                e if e > 0.0 && e < PI => target_output[1] = 1.0,
                e if e < 0.0 && e > -PI => target_output[0] = 1.0,
                _ => {}
            }
            target_output[2] = 0.0;
        }
        else if self.input[2] > 0.0 {
            match self.input[3] {
                e if e > 0.0 => target_output[0] = e,
                e if e < 0.0 => target_output[1] = -e,
                _ => {}
            }
            target_output[2] = 0.5;
        }
        else{
            target_output[0] = 0.0; //output[0];
            target_output[1] = 0.0; //output[1];
            target_output[2] = 1.0;
        }

        target_output
    }

}
