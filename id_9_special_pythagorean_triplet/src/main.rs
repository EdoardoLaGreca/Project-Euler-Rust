/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

// This function checks if `a`, `b` and `c` is a triplet
fn is_triplet(a: i32, b: i32, c: i32) -> bool {
	return a.pow(2) + b.pow(2) == c.pow(2)
}

fn main() {

	// a^2 + b^2 = c^2
	let mut a = 0;
	let mut b = 0;
	let mut c = 0;

	// This is the final result
	let triplet_product: i32;

	loop {

		// These nested if statements make possible to check all the combinations
		// It's inspired by the binary counting system
		if a < 1000 {
			a += 1;
		} else {
			if b < 1000 {
				b += 1;
				a = 0
			} else {
				c += 1;
				b = 0;
				a = 0;
			}
		}

		// If the sum of 'a' + `b` + `c` is 1000 and it's a triplet
		if a + b + c == 1000 && is_triplet(a, b, c) {

			// Assign to `triplet_product` `a` * `b` * `c` and break the loop
			triplet_product = a * b * c;
			break;
		}
	}

	println!("{}", triplet_product);
}