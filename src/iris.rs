#[derive(Debug, Clone)]
pub struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    iris: Vec<f64>,
    class: Vec<usize>,
}

impl Iris {
    pub fn new(
        sepal_length: f64,
        sepal_width: f64,
        petal_length: f64,
        petal_width: f64,
        class: Vec<usize>,
    ) -> Self {
        Self {
            sepal_length: sepal_length,
            sepal_width: sepal_width,
            petal_length: petal_length,
            petal_width: petal_width,
            iris: vec![sepal_length, sepal_width, petal_length, petal_width],
            class: class,
        }
    }

    pub fn normalize(&mut self) {
        let mut max = vec![
            self.sepal_length,
            self.sepal_width,
            self.petal_length,
            self.petal_width,
        ];
        max.sort_by(|a, b| a.partial_cmp(b).unwrap());
        max.reverse();

        let mut min = vec![
            self.sepal_length,
            self.sepal_width,
            self.petal_length,
            self.petal_width,
        ];
        min.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // println!("{:?}\n{:?}", max, min);

        self.sepal_length = (self.sepal_length - min[0]) / (max[0] - min[0]);
        self.sepal_width = (self.sepal_width - min[0]) / (max[0] - min[0]);
        self.petal_length = (self.petal_length - min[0]) / (max[0] - min[0]);
        self.petal_width = (self.petal_width - min[0]) / (max[0] - min[0]);
        self.iris = vec![
            self.sepal_length,
            self.sepal_width,
            self.petal_length,
            self.petal_width,
        ];
    }

    pub fn iris(&self) -> &Vec<f64> {
        &self.iris
    }

    pub fn class(&self) -> &Vec<usize> {
        &self.class
    }
}
