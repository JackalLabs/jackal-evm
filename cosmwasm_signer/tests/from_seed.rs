extern crate bip39;
extern crate bip32;
extern crate hex;

use bip39::{Language};
use bip32::{XPrv, DerivationPath, Seed, Mnemonic}; // WARNING: This is importing bip32 directly, not the one that cosmrs re-imported and wrapped 
use hex::ToHex;

use cosmrs::crypto::secp256k1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_key_pair_from_seed_phrase() {

        // NOTE: can pass some of this back into cosmwasm_signer/src/main.rs once you're done dissecting
        let seed_phrase = "fork draw talk diagram fragile online style lecture ecology lawn dress hat modify member leg pluck leaf depend subway grit trumpet tongue crucial stumble";
        let mnemonic = Mnemonic::new(seed_phrase, Default::default()).unwrap();
        let seed = mnemonic.to_seed("password");
        let root_xprv = XPrv::new(&seed).expect("failed to get root xprv");

        // ATOM HD path: m/44'/118'/0'/0/0  
        let child_path = "m/44'/118'/0'/0/0";
        // let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse().expect("no child_path")).expect("no child_xprv");

        let sender_private_key = secp256k1::SigningKey::derive_from_path(seed, &child_path.parse().expect("no child_path")).expect("failed to generate private key");

        let child_xpub = sender_private_key.public_key();

        let signing_key = sender_private_key;
        let verification_key = child_xpub;
        println!("{:?}", verification_key)
    }
}