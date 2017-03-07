#![allow(dead_code)]

extern crate allegro;
extern crate allegro_primitives;
extern crate allegro_font;
extern crate rand;

mod neuralnet;
mod vectorn;
mod trainingset;
mod simulator;
mod allegrodata;
mod bot;
mod utility;
mod ray;

use simulator::Simulator;

fn main() {
    const SCREEN_SIZE: (i32, i32) = (800, 600);
    const FIELD_SIZE: (i32, i32) = (SCREEN_SIZE.0 * 2, SCREEN_SIZE.1 * 2);
    const TICK_RATE: i32 = 30;

    let mut sim = match Simulator::new(SCREEN_SIZE, FIELD_SIZE, TICK_RATE) {
        Ok(e) => e,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    sim.mainloop();


}

/*let mut nn = NeuralNet::new(4, 4);
let mut trainingset = Trainingset::new();
let mut learning_rate: f64 = 2.0;
let cycles = 100;
let runs_per_cycle = 1000;

nn.randomize(-1.0, 1.0);

let mut input: Vec<Vec<f64>> = vec![    vec![0.0, 0.0, 0.0, 0.0],
                                        vec![0.0, 1.0, 0.0, 0.0],
                                        vec![1.0, 0.0, 0.0, 0.0],
                                        vec![1.0, 1.0, 0.0, 0.0]  ];

let mut target: Vec<Vec<f64>> = vec![   vec![0.0, 0.0, 0.0, 0.0],
                                        vec![1.0, 0.0, 0.0, 0.0],
                                        vec![1.0, 0.0, 0.0, 0.0],
                                        vec![0.0, 0.0, 0.0, 0.0]  ];

loop {
    let i = match input.pop() {
        Some(e) => e,
        None => break
    };

    let t = match target.pop() {
        Some(e) => e,
        None => break
    };

    trainingset.add_set(i, t);
}

for i in 0..cycles {
    let avg_error = nn.train(&trainingset, learning_rate, runs_per_cycle);

    println!("runs: {:06} | learning_rate: {:.2} | avg_error: {:.2e}", i * runs_per_cycle, learning_rate, avg_error);

    learning_rate *= 0.98;

    if avg_error < 1e-6 {
        break;
    }
}

for e in trainingset.iter() {
    let output = nn.feed_forward(&e.0);
    println!("XOR: {:?} -> {:?}", &e.0[0..2], &output[0]);
}*/
