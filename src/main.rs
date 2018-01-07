extern crate bitcoin;
extern crate secp256k1;
extern crate rand;

use bitcoin::util::address::{Address,Privkey};
use secp256k1::Secp256k1;
use secp256k1::key::SecretKey;
use bitcoin::network::constants::Network;
use rand::thread_rng;

fn main() {
	let secp = Secp256k1::new();
	let mut rng = thread_rng();
	let mysecret = SecretKey::new(&secp, &mut rng);
	let myprivkey = Privkey::from_key(Network::Bitcoin, mysecret, false);
	let myaddress: Address = myprivkey.to_address(&secp).unwrap();

	println!("Secret Key is {:?}", mysecret);
	println!("Private Key is {:?}", myprivkey.secret_key());
	println!("My Address is {:?}", myaddress);

}