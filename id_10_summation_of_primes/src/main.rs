/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

fn is_prime(n: i64) -> bool {

	// These numbers are too small to be used in the for loop
	match n {
		0 | 1  => return false,
		2 => return true,
		_ => (),
	}

	// For each number between 1 and n/2
	for i in 1..=(n/2) {

		// If n module i is 0 return false
		if n % i == 0 && n != i && i != 1 {
			return false;
		}
	}

	// If the for loop breaks without returning false, is prime
	return true;
}

fn main() {

	// Limit of prime numbers value
	let limit = 2_000_000;

	// Read the variable name and guess what is
	let mut primes_below_2million: Vec<u32> = Vec::new();

	// This is the problem's result
	let mut sum_of_primes: u128 = 0;

	for i in 0..=limit as u32 {

		// For each number from 0 to 2,000,000
		// if is prime add it to `primes_below_2million`
		if is_prime(i as i64) {
			primes_below_2million.push(i);
		}
	}

	// Sum all the items in `primes_below_2million` and put the result
	// inside of `sum_of_primes`
	for i in primes_below_2million.clone() {
		sum_of_primes += i as u128;
	}
	
	println!("{}", sum_of_primes);

}
