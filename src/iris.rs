use ndarray::{Array1, Array2, ArrayView1};

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
        let iris: Array2<f64> = match Array2::from_shape_vec((1, iris_vec.len()), iris_vec.to_vec())
        {
            Ok(result) => result,
            Err(error) => panic!(
                "Shape does not correspond to the number of elements in vector v {}!",
                error
            ),
        };

        let class_vec: Vec<f64> = class;
        let class: Array2<f64> =
            match Array2::from_shape_vec((1, class_vec.len()), class_vec.to_vec()) {
                Ok(result) => result,
                Err(error) => panic!(
                    "Shape does not correspond to the number of elements in vector v {}!",
                    error
                ),
            };

        Self {
            iris_vec,
            iris,
            class_vec,
            class,
        }
    }

    fn l2_norm(&mut self, x: ArrayView1<f64>) -> f64 {
        x.dot(&x).sqrt()
    }

    fn normalize_vec(&mut self, mut x: Array1<f64>) -> Vec<f64> {
        let norm = self.l2_norm(x.view());
        x.mapv_inplace(|e| e / norm);
        x.to_vec()
    }

    pub fn normalize(&mut self) {
        self.iris_vec = self.normalize_vec(Array1::from_vec(self.iris_vec.clone()));

        self.iris = match Array2::from_shape_vec((1, self.iris_vec.len()), self.iris_vec.to_vec()) {
            Ok(result) => result,
            Err(error) => panic!(
                "Shape does not correspond to the number of elements in vector v {}!",
                error
            ),
        };
    }
}
