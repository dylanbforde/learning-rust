use rand::Rng;

use crate::helper_functions::{sigmoid, derivative};

pub struct NeuralNetwork {
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
}

impl NeuralNetwork {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let weights = vec![rng.random_range(0.0..1.0), rng.random_range(0.0..1.0)];

        Self {
            weights,
            bias: rng.random_range(0.0..1.0),
            learning_rate: 0.1,
        }
    }

    pub fn predict(&self, input: &[f64; 2]) -> f64 {
        let mut sum = self.bias;
        for (i, weight) in self.weights.iter().enumerate() {
            sum += input[i] * weight;
        }

        sigmoid(sum)
    }

    pub fn train(&mut self, inputs: Vec<[f64; 2]>, outputs: Vec<f64>, epochs: usize) {
        for _ in 0..epochs {
            for (i, input) in inputs.iter().enumerate() {
                let output = self.predict(input);
                let error = outputs[i] - output;
                let delta = derivative(output);

                for j in 0..self.weights.len() {
                    self.weights[j] += self.learning_rate * error * input[j] * delta;
                }

                self.bias += self.learning_rate * error * delta;
            }
        }
    }
}