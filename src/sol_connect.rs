use anchor_client;
#[allow(unused_imports)]
pub use borsh::{BorshDeserialize, BorshSerialize};
pub use solana_client::rpc_client::RpcClient;
#[allow(unused_imports)]
pub use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Signature,
    signature::{Keypair, Signer},
    signer::EncodableKey,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;

/// Sign and submit a legacy transaction.
///
/// This method fully signs a transaction with all required signers, which
/// must be present in the `keypairs` slice.
///
/// # Panics
///
/// Panics when signing or signature verification fails.
///
/// # Examples
///
/// This example uses the [`solana_program_client`] crate.
///
/// ```

 #[derive(BorshSerialize, BorshDeserialize)]
 #[borsh(crate = "borsh")]
 pub struct NewAccount {
     slope: f64,
     intercept: f64
 }

pub fn save_data_to_solana(slope: f64, intercept: f64) {
     // create a Rpc client connection
     let url = "https://api.devnet.solana.com".to_string();
     let connection = RpcClient::new(url);
     let program_id = Pubkey::from_str("GsX4b44N2vkDjnZPLucGV7ou5qxADN2N6BZ7zU8vnJ1X").unwrap();
    //  let seed = b"new_account";
    // let (new_account, _) = Pubkey::find_program_address(&[seed], &program_id);
    let account_new = Keypair::new().pubkey();
     let payer = Keypair::read_from_file("src/wallet-keypair.json").unwrap();

     let instruction_name = "initialize";
    
     //  construct instruction data
     let instruction_data = NewAccount {
        slope,
        intercept ,
     };

     // setup signers
     let signers = &[&payer];

     // set up accounts
     let accounts = vec![
         AccountMeta::new(account_new, false),
         AccountMeta::new(payer.pubkey(), true),
     ];

     // call signed call
     let _tx_signature = signed_call(
         &connection,
         &program_id,
         &payer,
         signers,
         instruction_name,
         instruction_data,
         accounts,
     )
     .unwrap();
 }

pub fn signed_call(
    connection: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    signers: &[&Keypair],
    instruction_name: &str,
    instruction_data: NewAccount,
    accounts: Vec<AccountMeta>,
) -> Result<Signature, Box<dyn std::error::Error>>

{
    // get discriminant
    let instruction_discriminant = get_discriminant("global", instruction_name);

    // construct instruction
    let ix = Instruction::new_with_borsh(
        program_id.clone(),
        &(instruction_discriminant, instruction_data),
        accounts.clone(),
    );

    // get latest block hash
    let blockhash = connection.get_latest_blockhash()?;

    // construct message
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);

    //construct transaction
    let mut tx = Transaction::new_unsigned(msg);

    // sign transaction
    tx.sign(signers, tx.message.recent_blockhash);

    // send and confirm transaction
    let tx_signature = connection.send_and_confirm_transaction(&tx)?;

    Ok(tx_signature)
}

/// returns function signature
///
/// accepts name space and name function
pub fn get_discriminant(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_client::anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()
            [..8],
    );
    sighash
}