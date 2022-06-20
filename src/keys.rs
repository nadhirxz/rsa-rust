use rand::Rng;

fn gcd(x: i32, y: i32) -> i32 {
	let mut a = x;
	let mut b = y;

	while b != 0 {
		(a, b) = (b, a % b);
	}
	a
}

pub fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
	if a == 0 {
		(b, 0, 1)
	} else {
		let (g, x, y) = egcd(b % a, a);
		(g, y - (b / a) * x, x)
	}
}

pub fn modinverse(a: i32, m: i32) -> i32 {
	let (g, x, _) = egcd(a, m);

	assert!(g == 1, "modular inverse doesn't exist");

	(x % m + m) % m
}

pub fn generate(p: i32, q: i32) -> ((i32, i32), (i32, i32)) {
	assert!((p, q) != (0, 0), "failed to generate key");

	let n = p * q;
	let phi = (p - 1) * (q - 1);
	let mut e = rand::thread_rng().gen_range(1..phi);

	let mut g = gcd(e, phi);

	while g != 1 {
		e = rand::thread_rng().gen_range(1..phi);
		g = gcd(e, phi);
	}

	let d = modinverse(e, phi);

	return ((e, n), (d, n));
}
