use ndarray::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct Layer {
    neurons: Array2<f64>,
    weights: Array2<f64>,
    activation: String
}

impl Layer {
    pub fn flatten(input_shape: (usize, usize)) -> Self {
        let neurons: Array2<f64> = Array::default((1, (input_shape.0*input_shape.1)));
        let weights: Array2<f64> = Array::default((0, 0));
        let activation: String = match String::from_str("") {
            Ok(result) => result,
            Err(error) => panic!("There was a problem converting str to String: {:?}", error),
        };

        Self {
            neurons,
            weights,
            activation,
        }
    }

    pub fn dense(size: usize, activation: &str) -> Self {
        let neurons: Array2<f64> = Array::default((1, size));
        let weights: Array2<f64> = Array::default((0, 0));
        let activation: String = match String::from_str(activation) {
            Ok(result) => result,
            Err(error) => panic!("There was a problem converting str to String: {:?}", error),
        };

        Self {
            neurons,
            weights,
            activation,
        }
    }

    fn sigmoid(&mut self) {
        for neuron in self.neurons.iter_mut() {
            *neuron = 1_f64 / (1_f64 + -neuron.exp());
        }
    }

    pub fn neurons(&self) -> &Array2<f64> {
        &self.neurons
    }

    pub fn neurons_mut(&mut self, neurons: Array2<f64>) {
        self.neurons = neurons
    }

    pub fn weights(&self) -> &Array2<f64> {
        &self.weights
    }

    pub fn weights_mut(&mut self, weights: Array2<f64>) {
        self.weights = weights
    }
}
