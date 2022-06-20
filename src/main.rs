mod generate;
mod keys;
use rand::Rng;


fn main() {
	let number = rand::thread_rng().gen_range(100..10000);
	let (p, q) = generate::prime_pair(number);
	let (public, private) = keys::generate(p, q);

	println!("Public Key: {:?}", public);
	println!("Private Key: {:?}", private);
}
