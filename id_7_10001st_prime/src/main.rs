/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10,001st prime number?
*/

// Check if a number is prime
fn is_prime(n: i64) -> bool {

	// These numbers are too small to be used in the for loop
	match n {
		0 | 1  => return false,
		2 => return true,
		_ => (),
	}

	// For each number between 5 and n/2
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

	// Example:
	// 2 has position 1
	// 3 has position 2
	// 5 has position 3
	// 7 has position 4
	// etc...
	// Initialized at 0 so that the first iteration of the while loop
	// makes it become 1
	let mut prime_number_position = 0;

	// Current number used in while loop to increase prime_number_position
	let mut num = 2;

	// While position 10,001 is not reached
	// IDK why but it does not work without "+ 1" :(
	while prime_number_position <= 10_001 + 1 {

		// If the current number is prime
		if is_prime(num) {

			// Increase the prime position
			prime_number_position += 1;
		}

		// Don't increase num if prime_number_position is already at the 10_001th position
		if prime_number_position == 10_001 + 1 {
			break;
		}

		num += 1;
	}

	println!("{}", num);
}
