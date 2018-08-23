/*
The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
(1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and
the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural
numbers and the square of the sum.
*/

// Sum of squares
// start -> starting value (1, for example)
// end -> ending value (10, for example)
// result = 1^2 + 2^2 + 3^2 + ... + 10^2
fn sum_of_squares(start: i32, end: i32) -> i32 {

	// Output
	let mut result = 0;

	// For each number between start and end (including extremes)
	for i in start..=end {

		// Add the square of number to result
		result += i.pow(2);
	}

	result
}

// Square of sum
// start -> starting value (1, for example)
// end -> ending value (10, for example)
// result = (1 + 2 + 3 + ... + 10)^2
fn square_of_sum(start: i32, end: i32) -> i32 {

	// Output
	let mut result = 0;

	// For each number between start and end (including extremes)
	for i in start..=end {

		// Add the number to result
		result += i;
	}

	// Return square of result
	result.pow(2)
}


fn main() {

	// Start number and end number
	let start_n = 1;
	let end_n = 100;

	// The difference (as absolute value) between square of sum and sum of squares given start and end numbers
	let difference = (square_of_sum(start_n, end_n) - sum_of_squares(start_n, end_n)).abs();

	println!("{}", difference);
}
