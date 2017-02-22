use rand::distributions::{ Range, IndependentSample };
use rand;
use std::f64;

use vectorn::{ Vector2, Vector3 };
use trainingset::Trainingset;

pub struct NeuralNet {

    layers: usize,
    units: usize,
    weight: Vector3<f64>,
    bias: Vector2<f64>,
    unit_input: Vector2<f64>,
    unit_output: Vector2<f64>
}

fn activation(input: f64) -> f64 {
    1.0 / (1.0 + f64::consts::E.powf(-input))
}

fn activation_derivative(input: f64) -> f64 {
    let act = activation(input);
    act * (1f64 - act)
}

fn square_error(current: &[f64], target: &[f64]) -> f64 {
    let mut square_sum = 0f64;
    for i in 0..current.len() {
        square_sum += (target[i] - current[i]).powf(2f64);
    }
    0.5 * square_sum
}

impl NeuralNet {
    pub fn new(layers: usize, units: usize) -> NeuralNet {
        NeuralNet {
            layers: layers,
            units: units,
            weight: Vector3::new(0.0, layers, units, units),
            bias: Vector2::new(0.0, layers, units),
            unit_input: Vector2::new(0.0, layers, units),
            unit_output: Vector2::new(0.0, layers, units)
        }
    }

    fn get_unit_input(&self, layer: usize, unit: usize) -> f64{
        *self.unit_input.get(layer, unit)
    }

    fn set_unit_input(&mut self, value: f64, layer: usize, unit: usize) {
        *self.unit_input.get_mut(layer, unit) = value
    }

    fn mod_unit_input(&mut self, value: f64, layer: usize, unit: usize) {
        *self.unit_input.get_mut(layer, unit) += value
    }

    fn get_unit_output(&self, layer: usize, unit: usize) -> f64{
        *self.unit_output.get(layer, unit)
    }

    fn set_unit_output(&mut self, value: f64, layer: usize, unit: usize) {
        *self.unit_output.get_mut(layer, unit) = value
    }

    fn get_bias(&self, layer: usize, unit: usize) -> f64 {
        *self.bias.get(layer, unit)
    }

    fn mod_bias(&mut self, value: f64, layer: usize, unit: usize) {
        *self.bias.get_mut(layer, unit) += value;
    }

    fn activate_unit(&mut self, layer: usize, unit: usize) {
        let bias = self.get_bias(layer, unit);
        self.mod_unit_input(bias, layer, unit);
        let output = activation(self.get_unit_input(layer, unit));
        self.set_unit_output(output, layer, unit)
    }

    fn get_weight(&self, layer: usize, unit_src: usize, unit_dest: usize) -> f64{
        *self.weight.get(layer, unit_src, unit_dest)
    }

    fn mod_weight(&mut self, value: f64, layer: usize, unit_src: usize, unit_dest: usize) {
        *self.weight.get_mut(layer, unit_src, unit_dest) += value
    }

    fn clear_units(&mut self) {
        self.unit_input.clear(0.0);
        self.unit_output.clear(0.0);
    }

    pub fn randomize(&mut self, lower: f64, upper: f64) {
        self.randomize_weights(lower, upper);
        self.randomize_bias(lower, upper);
    }

    pub fn randomize_weights(&mut self, lower: f64, upper: f64) {
        let range = Range::new(lower, upper);
        let mut rng = rand::thread_rng();

        for e in self.weight.iter_mut() {
            *e = range.ind_sample(&mut rng);
        }
    }

    pub fn randomize_bias(&mut self, lower: f64, upper: f64) {
        let range = Range::new(lower, upper);
        let mut rng = rand::thread_rng();

        for e in &mut self.bias.iter_mut() {
            *e = range.ind_sample(&mut rng);
        }
    }

    pub fn feed_forward(&mut self, net_input: &Vec<f64>) -> Vec<f64> {
        self.clear_units();

        for unit in 0..self.units {
            self.set_unit_input(net_input[unit], 0, unit);
            self.set_unit_output(net_input[unit], 0, unit);
        }

        for layer in 0..self.layers - 1 {
            for unit in 0..self.units {
                if layer > 0 {
                    self.activate_unit(layer, unit);
                }
                for dest_unit in 0..self.units {
                    let input = self.get_unit_output(layer, unit) * self.get_weight(layer, unit, dest_unit);
                    self.mod_unit_input(input, layer + 1, dest_unit)
                }
            }
        }

        let last_layer = self.layers - 1;
        let mut output = vec![0f64; self.units];
        for unit in 0..self.units {
            self.activate_unit(last_layer, unit);
            output[unit] = self.get_unit_output(last_layer, unit);
        }
        output
    }

    pub fn backpropagate(&mut self, target: &Vec<f64>, learning_rate: f64) -> f64 {

        let mut delta = vec![0f64; self.layers * self.units];
        let last_layer = self.layers - 1;

        for unit in 0..self.units {
            delta[last_layer * self.units + unit] = activation_derivative(self.get_unit_input(last_layer, unit)) * (target[unit] - self.get_unit_output(last_layer, unit));
        }

        for layer in (0..self.layers - 1).rev() {
            for unit in 0..self.units {
                let input = self.get_unit_input(layer, unit);
                //let output = self.get_unit_output(layer, unit);

                let mut delta_sum = 0f64;
                {
                    let prev_delta = &delta[(layer + 1) * self.units..(layer + 2) * self.units];
                    for unit_dest in 0..self.units {
                        delta_sum += prev_delta[unit_dest] * self.get_weight(layer, unit, unit_dest);
                    }
                }

                delta[layer * self.units + unit] = activation_derivative(input) * delta_sum;
            }
        }

        for layer in 0..self.layers - 1 {
            for unit in 0..self.units {
                for unit_dest in 0..self.units {
                    let output = self.get_unit_output(layer, unit);
                    let change = learning_rate * delta[(layer + 1) * self.units + unit_dest] * output;
                    self.mod_weight(change, layer, unit, unit_dest);
                }

                //TODO Check if correct. It seems it has no negative impact.
                let bias = self.get_bias(layer, unit);
                let change = learning_rate * delta[(layer + 1) * self.units + unit] * bias;
                self.mod_bias(change, layer, unit);
            }
        }

        square_error(self.unit_output.get_row(last_layer), target)
    }

    pub fn train(&mut self, trainingset: &Trainingset, learning_rate: f64, runs: u32) -> f64 {
        let mut avg_error = 0f64;
        for _ in 0..runs {
            let mut total_error = 0f64;
            for ts in trainingset.iter() {
                self.feed_forward(&ts.0);
                total_error += self.backpropagate(&ts.1, learning_rate);
            }
            avg_error = total_error / trainingset.get_set_count() as f64;
        }
        avg_error
    }
}
