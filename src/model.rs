use rand::distributions::Uniform;
use ndarray::Array2;
use ndarray_rand::RandomExt;
use std::str::FromStr;

use crate::layer::Layer;

#[derive(Debug)]
pub struct Model {
    layers: Vec<Layer>,
    optimizer: String,
    loss: String,
    metrics: Vec<String>,
}

impl Model {
    pub fn new() -> Self {
        let optimizer: String = match String::from_str("") {
            Ok(result) => result,
            Err(error) => panic!("There was a problem converting str to String: {:?}", error),
        };

        let loss: String = match String::from_str("") {
            Ok(result) => result,
            Err(error) => panic!("There was a problem converting str to String: {:?}", error),
        };

        let metrics: Vec<String> = Vec::new();

        Model {
            layers: Vec::<Layer>::new(),
            optimizer,
            loss,
            metrics,
        }
    }

    pub fn add(&mut self, layer: Layer) {
        let length: usize = self.layers.len();
        if length > 0 {
            let rows: usize = match self.layers.last() {
                Some(result) => result.neurons().cols(),
                None => panic!("There is no layer added to the model"),
            };
            let cols: usize = layer.neurons().cols();

            match self.layers.last_mut() {
                Some(result) => result.weights_mut(Array2::random((rows, cols), Uniform::new(0., 1.))),
                None => panic!("There is no layer added to the model"),
            }
        }
        self.layers.push(layer);
    }

    pub fn compile(&mut self, optimizer: &str, loss: &str, metrics: Vec<&str>) {
        // Retrieve compilation informations
        self.optimizer = match String::from_str(optimizer) {
            Ok(result) => result,
            Err(error) => panic!("There was a problem converting str to String: {:?}", error),
        };

        self.loss = match String::from_str(loss) {
            Ok(result) => result,
            Err(error) => panic!("There was a problem converting str to String: {:?}", error),
        };

        for metric in metrics {
            self.metrics.push(match String::from_str(metric) {
                Ok(result) => result,
                Err(error) => panic!("There was a problem converting str to String: {:?}", error),
            })
        }
    }

    pub fn fit(training_set: Vec<f64>, set_labels: Vec<&str>, epochs: usize) {
        unimplemented!();
    }

    pub fn evaluate(test_set: Vec<f64>, set_labels: Vec<&str>) -> (usize, usize) {
        unimplemented!();
    }

    pub fn predict<T>(prediction_set: T) -> Array2<f64> {
        unimplemented!();
    }

    fn adam_optimizer() {
        let learning_rate = 0.001;
        let beta_1 = 0.9;
        let beta_2 = 0.999;
        let epsilon = 10_f64.powf(-8_f64);
    }
}
