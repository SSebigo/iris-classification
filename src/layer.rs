use ndarray::prelude::*;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct Layer {
    pub neurons: Array2<f64>,
    pub weights: Array2<f64>,
    activation: String,
}

impl Layer {
    pub fn flatten(input_shape: (usize, usize)) -> Self {
        let neurons: Array2<f64> = Array::default((1, (input_shape.0 * input_shape.1)));
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

    pub fn activate(&mut self) {
        match self.activation.as_str() {
            "sigmoid" => self.sigmoid(),
            "fast_sigmoid" => self.fast_sigmoid(),
            "tanh" => self.tanh(),
            _ => {}
        }
    }

    fn sigmoid(&mut self) {
        for neuron in self.neurons.iter_mut() {
            *neuron = 1_f64 / (1_f64 + (-*neuron).exp());
        }
    }

    fn fast_sigmoid(&mut self) {
        for neuron in self.neurons.iter_mut() {
            *neuron = *neuron / (1_f64 + neuron.abs());
        }
    }

    fn sigmoid_prime(&mut self) {
        unimplemented!();
    }

    fn tanh(&mut self) {
        for neuron in self.neurons.iter_mut() {
            *neuron = neuron.tanh();
        }
    }
}
