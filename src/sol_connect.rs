use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::Transaction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::message::Message;
use solana_sdk::system_instruction;
use solana_sdk::signer::Signer;

async fn save_to_solana(model_params: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let payer = Keypair::new();
    let pubkey = Pubkey::new_unique();
    
    // Create a transaction to save data to Solana
    let instruction = system_instruction::transfer(&payer.pubkey(), &pubkey, 1_000_000_000);
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    
    // Sign transaction
    transaction.sign(&[&payer], client.get_latest_blockhash().await?);

    // Send transaction
    let signature = client.send_and_confirm_transaction(&transaction).await?;
    println!("Transaction signature: {}", signature);

    Ok(())
}