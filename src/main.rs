extern crate bitcoin;
extern crate secp256k1;
extern crate rand;
extern crate crypto;
extern crate hex;

use bitcoin::util::address::{Address,Privkey};
use secp256k1::Secp256k1;
use secp256k1::key::SecretKey;
use bitcoin::network::constants::Network;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
	let secp = Secp256k1::new();
	let mut hasher = Sha256::new();
	hasher.input_str("1972");
	let mut newhash: [u8; 32] = [0; 32];
	hasher.result(&mut newhash);
	let mysecret = match SecretKey::from_slice(&secp, &newhash[0..32]) {
		Ok(key) => { key },
		Err(e) => { panic!("{}", e) }
	};
	let myprivkey = Privkey::from_key(Network::Bitcoin, mysecret, false);
	let myaddress: Address = myprivkey.to_address(&secp).unwrap();

	println!("Secret Key is {:?}", mysecret);
	println!("Private Key is {:?}", myprivkey.secret_key());
	println!("My Address is {:?}", myaddress);

}