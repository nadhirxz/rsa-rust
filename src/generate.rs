
fn sieve(primes: &mut Vec<i32>, factor: i32) {
	let mut i = 0;
	while i < primes.len() {
		let value = primes[i];
		if value != factor {
			if value % factor == 0 {
				primes.remove(i);
			}
		}
		i += 1;
	}
}

fn generate_primes(n: i32) -> Vec<i32> {
	let mut primes: Vec<i32> = Vec::new();

	for i in 2..=n {
		primes.push(i);
	}

	let mut i = 0;
	while i < primes.len() {
		let factor = primes[i];
		sieve(&mut primes, factor);
		i += 1;
	}

	return primes;
}

pub fn prime_pair(n: i32) -> (i32, i32) {
	let primes = generate_primes(n);
	let mut i = 0;
	while primes[i] <= n / 2 {
		let diff = n - primes[i];

		if primes.contains(&diff) {
			return (primes[i], diff);
		}
		i += 1;
	}

	return (0, 0);
}