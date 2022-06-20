mod generate;
use rand::Rng;


fn main() {
	let number = rand::thread_rng().gen_range(100..10000);
	let (p, q) = generate::prime_pair(number);
}
