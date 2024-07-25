extern crate bip39;
extern crate bip32;
extern crate hex;

use bip39::{Language};
use bip32::{XPrv, DerivationPath, Seed, Mnemonic};
use hex::ToHex;

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

        let child_path = "m/0/2147483647'/1/2147483646'";
        let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse().expect("no child_path")).expect("no child_xprv");

        let child_xpub = child_xprv.public_key();

        let signing_key = child_xprv.private_key();
        let verification_key = child_xpub.public_key();
        println!("{:?}", verification_key)


    }







}