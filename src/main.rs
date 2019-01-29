mod iris;
mod data;
mod layer;
mod model;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use data::Data;
use layer::Layer;
use model::Model;

fn main() -> io::Result<()> {
    let mut file = File::open("./iris.data")?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut data: Data = Data::new();
    data.retrieve_data(buffer);

    // println!("data: {:#?}", data);

    let (train_set, test_set) = data.data().split_at((150.0*0.80) as usize);
    let (train_set, test_set) = (train_set.to_vec(), test_set.to_vec());

    // println!("train set: {:#?}\ntest set: {:#?}", train_set, test_set);

    let labels: Vec<&str> = vec!["Iris-setosa", "Iris-versicolor", "Iris-virginica"];

    let mut model: Model = Model::new();
    model.add(Layer::flatten((2, 2)));
    model.add(Layer::dense(512, "fast_sigmoid"));
    model.add(Layer::dense(3, "fast_sigmoid"));

    model.compile("adam", "sparse_categorical_crossentropy", vec!["accuracy"]);

    model.fit(train_set, labels, 5);

    println!("model: {:#?}", model);

    Ok(())
}
