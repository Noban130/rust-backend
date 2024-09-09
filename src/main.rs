use std::error::Error;
// use std::env;
pub mod traning_example;
// // pub mod sol_connect;
pub mod data_preprocess;
// use std::iter::repeat_with;
// #[tokio::main]
fn main() -> Result<(), Box<dyn Error>>{
    // let current_dir = env::current_dir()?;
    // println!("Current working directory: {:?}", current_dir);
    let (mut x_value, mut y_value): (Vec<i64>, Vec<f64>) = (Vec::new(), Vec::new());
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
    }
    Ok(())
    // let data: Vec<i32> = (x, y); // Replace with your data
    // let min_chunk_size = 5;
    // let max_chunk_size = 15;

    // let chunks = split_data_randomly(&data, min_chunk_size, max_chunk_size);

    // for (i, chunk) in chunks.iter().enumerate() {
    //     println!("Chunk {}: {:?}", i, chunk);
    //     let model = train_model_in_chunks(x, y, chunk_size);    
    //     save_to_solana(&model).await?;
    // }
        
}