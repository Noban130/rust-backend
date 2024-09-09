use csv::ReaderBuilder;
use std::error::Error;
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use rand::seq::SliceRandom;

fn read_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let mut x_values = Vec::new();
    let mut y_values = Vec::new();

    for result in reader.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        x_values.push(x);
        y_values.push(y);
    }

    Ok((x_values, y_values))
}

fn train_model_in_chunks(x: Vec<f64>, y: Vec<f64>, chunk_size: usize) -> LinearRegression<f64> {
    let mut rng = rand::thread_rng();
    let mut x_chunked = x.chunks(chunk_size).collect::<Vec<_>>();
    let mut y_chunked = y.chunks(chunk_size).collect::<Vec<_>>();

    // Shuffle chunks
    x_chunked.shuffle(&mut rng);
    y_chunked.shuffle(&mut rng);

    let mut model = LinearRegression::default();

    for (x_chunk, y_chunk) in x_chunked.iter().zip(y_chunked.iter()) {
        let dataset = linfa::Dataset::from((x_chunk.to_vec(), y_chunk.to_vec()));
        model.fit(&dataset).unwrap();
    }

    model
}