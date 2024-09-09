use std::error::Error;
pub mod traning_example;
pub mod data_preprocess;
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{Array, Array1};
use linfa::DatasetBase;
// use std::env;
// // pub mod sol_connect;
// use std::iter::repeat_with;
// #[tokio::main]
fn main() -> Result<(), Box<dyn Error>>{
    // let current_dir = env::current_dir()?;
    // println!("Current working directory: {:?}", current_dir);
    let (mut x_value, mut y_value): (Vec<f64>, Vec<f64>) = (Vec::new(), Vec::new());
    match traning_example::read_csv("src/study.csv") {
        Ok((x, y)) => {
            // println!("{:?}", (x, y));
            x_value = x;
            y_value = y
        },
        Err(e) => {
            eprintln!("Error reading CSV: {}", e);
            
        }
    }
    let min_chunk_size = 2;
    let max_chunk_size = 10;

    let chunks = data_preprocess::split_data_randomly(x_value, y_value, min_chunk_size, max_chunk_size);

    for (i, (chunk_x, chunk_y)) in chunks.iter().enumerate() {
        println!("Chunk {}: x = {:?}, y = {:?}", i + 1, chunk_x, chunk_y);
        // Convert x and y to ndarray arrays for linfa
        let x_arr: Array1<f64> = Array::from_vec(chunk_x.clone());
        let y_arr: Array1<f64> = Array::from_vec(chunk_y.clone());
        let dataset = DatasetBase::new(x_arr, y_arr);
        // Train linear regression model
        let model = LinearRegression::default()
            .fit(&dataset)
            .expect("Failed to fit linear regression model");

        // Extract and print model parameters
        let intercept = model.intercept();
        let coefficients = model.params();

        println!("Chunk {}: slope = {}, intercept = {}", i + 1, coefficients[0], intercept);
    }
    Ok(())
        
}