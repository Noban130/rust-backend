use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::{Keypair, Signer}, signer::EncodableKey, system_instruction, system_program, transaction::Transaction
};
use solana_program::instruction::Instruction;
use borsh::{BorshSerialize, BorshDeserialize};
use std::str::FromStr;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NewAccount {
    pub slope: f64,
    pub intercept: f64,
}

pub async fn save_data_to_solana(slope : f64, intercept : f64) -> Result<(), Box<dyn std::error::Error>> {
    // Create an RPC client to interact with Solana devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    // Load your wallet keypair (ensure you have funds in the wallet for transaction fees)
    let payer = Keypair::read_from_file("src/wallet-keypair.json")?;
    
    // Program ID (replace with your deployed program's ID)
    let program_id = Pubkey::from_str("GsX4b44N2vkDjnZPLucGV7ou5qxADN2N6BZ7zU8vnJ1X")?;
    
    // Create a new account for storing slope and intercept
    let new_account = Keypair::new();
    let space = 24; // Space for f64 (8 bytes for slope + 8 bytes for intercept)
    
    // Minimum balance for rent exemption
    let lamports = client.get_minimum_balance_for_rent_exemption(space)?;
    
    // Create an account initialization transaction
    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        lamports,
        space as u64,
        &program_id,
    );
    println!("payer:{:?}", create_account_instruction);

    // Define the slope and intercept values
    let slope: f64 = slope;
    let intercept: f64 = intercept;

    // Serialize the data to be sent to the on-chain account
    let new_account_data = NewAccount { slope, intercept };
    let serialized_data = new_account_data.try_to_vec()?;

    // Create the instruction to call the initialize method of the Solana program
    let initialize_instruction = Instruction {
        program_id,
        accounts: vec![
            solana_program::instruction::AccountMeta::new(new_account.pubkey(), false),
            solana_program::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_program::instruction::AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: serialized_data,
    };

    // Create the transaction
    let mut transaction = Transaction::new_with_payer(
        &[create_account_instruction, initialize_instruction],
        Some(&payer.pubkey()),
    );

    // Get the recent blockhash
    let recent_blockhash = match client.get_latest_blockhash() {
        Ok(hash) => hash,
        Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get latest blockhash"))),
    };
    println!("{:?}", recent_blockhash);
    // Sign the transaction with the payer and new account
    transaction.sign(&[&payer, &new_account], recent_blockhash);

    // Send and confirm the transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction signature: {}", signature);

    Ok(())
}