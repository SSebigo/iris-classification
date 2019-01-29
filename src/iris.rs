use ndarray::Array2;

#[derive(Debug, Clone)]
pub struct Iris {
    iris_vec: Vec<f64>,
    pub iris: Array2<f64>,
    class_vec: Vec<f64>,
    pub class: Array2<f64>,
}

impl Iris {
    pub fn new(
        sepal_length: f64,
        sepal_width: f64,
        petal_length: f64,
        petal_width: f64,
        class: Vec<f64>,
    ) -> Self {
        let iris_vec: Vec<f64> = vec![sepal_length, sepal_width, petal_length, petal_width];
        let iris: Array2<f64> = match Array2::from_shape_vec((1, iris_vec.len()), iris_vec.to_vec()) {
            Ok(result) => result,
            Err(error) => panic!("Shape does not correspond to the number of elements in vector v {}!", error),
        };

        let class_vec: Vec<f64> = class;
        let class: Array2<f64> = match Array2::from_shape_vec((1, class_vec.len()), class_vec.to_vec()) {
            Ok(result) => result,
            Err(error) => panic!("Shape does not correspond to the number of elements in vector v {}!", error),
        };

        Self {
            iris_vec,
            iris,
            class_vec,
            class,
        }
    }

    pub fn normalize(&mut self) {
        let mut max = self.iris_vec.clone();
        max.sort_by(|a, b| a.partial_cmp(b).unwrap());
        max.reverse();

        let mut min = self.iris_vec.clone();
        min.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // println!("{:?}\n{:?}", max, min);

        self.iris_vec[0] = (self.iris_vec[0] - min[0]) / (max[0] - min[0]);
        self.iris_vec[1] = (self.iris_vec[1] - min[0]) / (max[0] - min[0]);
        self.iris_vec[2] = (self.iris_vec[2] - min[0]) / (max[0] - min[0]);
        self.iris_vec[3] = (self.iris_vec[3] - min[0]) / (max[0] - min[0]);

        self.iris = match Array2::from_shape_vec((1, self.iris_vec.len()), self.iris_vec.to_vec()) {
            Ok(result) => result,
            Err(error) => panic!("Shape does not correspond to the number of elements in vector v {}!", error),
        };
    }
}
