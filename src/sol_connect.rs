use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_program;
use solana_sdk::message::Message;
use solana_sdk::system_instruction;
use solana_sdk::signer::Signer;
use solana_sdk::instruction::Instruction;
use solana_sdk::instruction::AccountMeta;
use bincode;
// use std::error::Error;

pub async fn save_to_solana(model_params: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new("https://rpc.ankr.com/solana_devnet".to_string());
    
    // Create payer keypair
    let payer = Keypair::new();

    // Generate a keypair for the new account that will hold the data
    let data_account = Keypair::new();

    // Serialize the model_params using bincode
    let serialized_data = bincode::serialize(&model_params)?;

    // Calculate how much space we need for storing the f64 array
    let space = serialized_data.len();

    // Minimum balance required for rent exemption
    let lamports = client.get_minimum_balance_for_rent_exemption(space)?;

    // Create an account with enough space to hold the serialized data
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),               // Funding account
        &data_account.pubkey(),        // New account to hold data
        lamports,                      // Amount of lamports to fund for rent exemption
        space as u64,                  // Space in bytes needed to store the data
        &system_program::ID,           // Owner of the account (can be your custom program)
    );

    // Build a transaction to create the account
    let message = Message::new(&[create_account_ix], Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);

    // Get recent blockhash
    let blockhash = client.get_latest_blockhash()?;

    // Sign the transaction
    transaction.sign(&[&payer, &data_account], blockhash);

    // Send and confirm the transaction
    client.send_and_confirm_transaction(&transaction)?;

    // At this point, the new account has been created with space to store the model parameters
    // Now, we'll write the serialized data into the account using a custom instruction
    let write_data_instruction = Instruction::new_with_bincode(
        system_program::ID, // This should be your custom program's ID
        &serialized_data,   // Serialized data to write
        vec![
            AccountMeta::new(data_account.pubkey(), false),
            AccountMeta::new(payer.pubkey(), true),
        ],
    );

    // Build the message with the write instruction
    let write_message = Message::new(&[write_data_instruction], Some(&payer.pubkey()));
    let mut write_transaction = Transaction::new_unsigned(write_message);

    // Sign the transaction
    write_transaction.sign(&[&payer], blockhash);

    // Send and confirm the transaction
    let signature = client.send_and_confirm_transaction(&write_transaction)?;

    println!("Data stored in account: {}, Transaction signature: {}", data_account.pubkey(), signature);

    Ok(())
}