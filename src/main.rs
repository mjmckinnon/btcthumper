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
use std::env;

fn main() {
	// Parse command line arguments
	let args: Vec<String> = env::args().collect();
	let myrange: (u32, u32) = match args.len() {
		3 => { (args[1].parse().unwrap(), args[2].parse().unwrap() ) },
		_ => panic!("Error. Usage: btcthumper <startval> <endval>")
	};
	// Loop across our range and start generating
	for n in myrange.0..(myrange.1+1) {
		let secp = Secp256k1::new();
		let mut hasher = Sha256::new();
		hasher.input_str(&n.to_string()[..]);
		let mut newhash: [u8; 32] = [0; 32];
		hasher.result(&mut newhash);
		let mysecret = match SecretKey::from_slice(&secp, &newhash[0..32]) {
			Ok(key) => { key },
			Err(e) => { panic!("{}", e) }
		};
		let myprivkey = Privkey::from_key(Network::Bitcoin, mysecret, false);
		let myaddress: Address = myprivkey.to_address(&secp).unwrap();
		println!("{:?},s2({})", myaddress, n);		
	}
}