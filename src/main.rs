pub mod traning;
pub mod sol_connect;
pub mod data_preprocess;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (x, y) = read_csv("data/simple_linear_regression.csv")?;
    
    let model = train_model_in_chunks(x, y, chunk_size);

    // Example to save model params (this part needs to be customized)
    let model_params = vec![/* Model parameters here */];
    save_to_solana(&model_params).await?;

    Ok(())
}