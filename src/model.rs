use ndarray::{Array, Array2};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use std::str::FromStr;

use crate::iris::Iris;
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
                Some(result) => result.neurons.cols(),
                None => panic!("There is no layer added to the model!"),
            };
            let cols: usize = layer.neurons.cols();

            match self.layers.last_mut() {
                Some(result) => result.weights = Array2::random((rows, cols), Uniform::new(0., 1.)),
                None => panic!("There is no layer added to the model!"),
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

    pub fn fit(&mut self, training_set: Vec<Iris>, set_labels: Vec<&str>, epochs: usize) {
        for epoch in 0..epochs {
            println!("Epoch {}/{}", epoch + 1, epochs);
            let mut correct_predict: usize = 0;
            let mut average_cost: f64 = 0_f64;
            for iris in training_set.iter() {
                for i in 0..self.layers.len() {
                    match i {
                        0 => { self.layers[i].neurons = iris.iris.clone(); }
                        _ => {
                            let matrix_1: &Array2<f64> = &self.layers[i-1].neurons;
                            let matrix_2: &Array2<f64> = &self.layers[i-1].weights;
                            self.layers[i].neurons = matrix_1.dot(matrix_2);
                            self.layers[i].activate();
                        }
                    }
                }

                // Increment correct_predict if prediction is correct...
                let index1: usize = 0;
                let index2: usize = 0;
                // if 

                if index1 == index2 { correct_predict += 1; };

                let mut prediction_error: f64 = 0_f64;
                match self.loss.as_str() {
                    "loss" => {
                        match self.layers.last() {
                            Some(result) => {
                                println!("prediction: {:#?}", result.neurons);
                                prediction_error = self.loss(result.neurons.clone(), iris.class.clone());
                                average_cost += prediction_error;
                            },
                            None => panic!("There is no layer added to the model!"),
                        }
                    },
                    "sparse_categorical_crossentropy" => {}
                    _ => {},
                }

                println!("prediction error: {:#?}", prediction_error);
            }

            average_cost = average_cost/training_set.len() as f64;
            let accuracy: f64 = (correct_predict/training_set.len()) as f64;

            match self.metrics[0].as_str() {
                "accuracy" => {
                    println!("loss: {} - acc: {}", average_cost, accuracy);
                },
                _ => {}
            }
        }
    }

    pub fn evaluate(test_set: Vec<f64>, set_labels: Vec<&str>) -> (usize, usize) {
        unimplemented!();
    }

    pub fn predict<T>(prediction_set: T) -> Array2<f64> {
        unimplemented!();
    }

    fn adam_optimizer(&mut self) {
        let learning_rate = 0.001;
        let beta_1 = 0.9;
        let beta_2 = 0.999;
        let epsilon = 10_f64.powf(-8_f64);
    }

    fn gradient_descent(&mut self) {
        unimplemented!();
    }

    fn loss(&mut self, prediction: Array2<f64>, target: Array2<f64>) -> f64 {
        (prediction - target).mapv(|v| v.powi(2)).sum()
    }

    fn mean_squared_error(&mut self, prediction: Array2<f64>, target: Array2<f64>) -> Array2<f64> {
        prediction - target
    }
}
