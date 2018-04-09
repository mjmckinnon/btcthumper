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
use std::mem::transmute;


fn main() {
	// Parse command line arguments
	let args: Vec<String> = env::args().collect();
	let myrange: (u8, u64, u64) = match args.len() {
		4 => { (args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()) },
		_ => { panic!("Error. Usage: btcthumper <mode> <startval> <endval>") }
	};
	let mode = myrange.0;
	let startval: u64 = myrange.1;
	let endval: u64 =
		// If the second number is smaller, add to the first and make that the end
		if myrange.2 <= myrange.1 {
			myrange.1 + myrange.2
		}
		else
		{
			// we add one so it finishes where you expect
		    myrange.2 + 1
		};
	// Loop across our range and start generating
	//static SEED: u64 = 8682522807148012 * 181783497276652981;
	static SEED: u64 = 8006678197202707420;
	for n in startval..endval {
		let secp = Secp256k1::new();
		let mut hasher = Sha256::new();
		let mut newhash: [u8; 32] = [0; 32];
		match mode {
			1 => { // then we are just doing a number
				   let nbytes: [u8; 8] = unsafe { transmute(n) };
				   newhash[24..].clone_from_slice(&nbytes); },
			2 => { // then we'll do a java.util.Random crack
				   let randval = n ^ SEED;
				   let nbytes: [u8; 8] = unsafe { transmute(randval) };
				   newhash[24..].clone_from_slice(&nbytes); },
			_ => { // then we are hasing as a string only
				   hasher.input_str(&n.to_string()[..]);
				   hasher.result(&mut newhash); },
		}
		let mysecret = match SecretKey::from_slice(&secp, &newhash[0..32]) {
			Ok(key) => { key },
			Err(e) => { panic!("{}", e) }
		};
		let myprivkey = Privkey::from_key(Network::Bitcoin, mysecret, false);
		let myaddress: Address = myprivkey.to_address(&secp).unwrap();
		println!("{:?},m{}({})", myaddress, mode, n);	
	};
}
