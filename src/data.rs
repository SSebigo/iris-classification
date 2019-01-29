use std::str::FromStr;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::iris::Iris;

#[derive(Debug)]
pub struct Data {
    data: Vec<Iris>,
}

impl Data {
    pub fn new() -> Self {
        Data { data: Vec::new() }
    }

    pub fn retrieve_data(&mut self, raw_data: String) {
        let lines = raw_data.lines();

        for line in lines {
            // println!("line: {:?}", line);
            let line_data: Vec<&str> = line.split(",").collect();
            let sepal_length = f64::from_str(line_data[0]);
            let sepal_length = match sepal_length {
                Ok(sepal_length) => sepal_length,
                Err(error) => panic!("There was a problem converting str to f64: {:?}", error),
            };
            let sepal_width = f64::from_str(line_data[1]);
            let sepal_width = match sepal_width {
                Ok(sepal_width) => sepal_width,
                Err(error) => panic!("There was a problem converting str to f64: {:?}", error),
            };
            let petal_length = f64::from_str(line_data[2]);
            let petal_length = match petal_length {
                Ok(petal_length) => petal_length,
                Err(error) => panic!("There was a problem converting str to f64: {:?}", error),
            };
            let petal_width = f64::from_str(line_data[3]);
            let petal_width = match petal_width {
                Ok(petal_width) => petal_width,
                Err(error) => panic!("There was a problem converting str to f64: {:?}", error),
            };

            let class: Vec<f64> = match line_data[4] {
                "Iris-setosa" => vec![1_f64, 0_f64, 0_f64],
                "Iris-versicolor" => vec![0_f64, 1_f64, 0_f64],
                "Iris-virginica" => vec![0_f64, 0_f64, 1_f64],
                _ => vec![0_f64, 0_f64, 0_f64],
            };

            let mut new_iris: Iris =
                Iris::new(sepal_length, sepal_width, petal_length, petal_width, class);
            new_iris.normalize();
            self.data.push(new_iris);
        }
        self.data.shuffle(&mut thread_rng());
    }

    pub fn data(&self) -> &Vec<Iris> {
        &self.data
    }
}
