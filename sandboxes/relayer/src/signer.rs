use cosmrs::{
    bip32::Mnemonic, 
    crypto::secp256k1, 
    AccountId
};

pub(crate)  fn generate_account_id_from_seed(seed_phrase: &str) -> Result<(AccountId, secp256k1::SigningKey), Box<dyn std::error::Error>> {
    // Generate mnemonic
    let mnemonic = Mnemonic::new(&seed_phrase, Default::default())?;
    
    // Convert mnemonic to seed
    let seed = mnemonic.to_seed(""); 

    // jackal's derivation path
    let derivation_path = "m/44'/118'/0'/0/0";
    
    // Derive the sender's private key
    let sender_private_key = secp256k1::SigningKey::derive_from_path(seed, &derivation_path.parse()?)?;
    
    // Get the corresponding public key
    let public_key = sender_private_key.public_key();

    // Generate the sender's account ID
    let sender_account_id = public_key.account_id("jkl")?;

    Ok((sender_account_id, sender_private_key))
}