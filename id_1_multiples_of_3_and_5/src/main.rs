/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
*/

fn main() {

	// Total sum
	let mut sum: u32 = 0;

	// For each number between 0 and 1000
	for i in (0 as u32)..(1000 as u32) {

		// If index is a multiple of 3 or 5, add it to sum
		if i % 3 == 0 || i % 5 == 0 {
			sum += i;
		}
	}

	println!("Result = {}", sum);
}
