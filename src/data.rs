use crate::iris::Iris;
use std::str::FromStr;

#[derive(Debug)]
pub struct Data {
    data: Vec<Iris>,
}

impl Data {
    pub fn new() -> Data {
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
            let class = String::from_str(line_data[4]);
            let class = match class {
                Ok(class) => class,
                Err(error) => panic!("There was a problem converting str to String: {:?}", error),
            };

            let mut new_iris: Iris =
                Iris::new(sepal_length, sepal_width, petal_length, petal_width, class);
            new_iris.normalize();
            self.data.push(new_iris);
        }
    }
}
