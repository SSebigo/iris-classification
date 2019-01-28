mod iris;
mod data;
mod layer;
mod model;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use data::Data;

fn main() -> io::Result<()> {
    let mut file = File::open("./iris.data")?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut data: Data = Data::new();
    data.retrieve_data(buffer);

    println!("data: {:?}", data);

    Ok(())
}
