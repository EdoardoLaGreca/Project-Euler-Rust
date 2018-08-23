/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn main() {

	// Contains the current number to check
	// The answer can't be below 20
	let mut num: i128 = 20;

	loop {

		// Is the current number valid?
		// This variable changes if num isn't divisible
		// by one or more numbers between 1 and 20.
		// If this variable reaches the end of the loop with
		// the value true, then the number is correct.
		let mut number_is_valid = true;

		// For each number between 1 and 20 (including extremes)
		for i in 1..=20 {

			// If num isn't divisible by all the numbers, set number_is_valid as false
			// and exit the for loop
			// If this if statement is not executed in any iteration, then it's the correct number.
			if !(num % i == 0) {
				number_is_valid = false;
				break;
			}
		}

		// If number_is_valid is true, means that is divisible by all the numbers
		// between 1 and 20 so it must be the correct answer.
		if number_is_valid {
			break;
		}

		// Increase num
		num += 1;
	}

	// It will take a while to find the result,
	// by the way the algorithm works pretty well.
	println!("{}", num);
}
