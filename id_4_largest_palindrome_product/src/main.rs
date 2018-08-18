/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.
*/

// Check if string s is palindrome
fn is_palindrome(s: String) -> bool {

	// Reversed string s
	let mut reversed_s = String::new();

	// Reverse s
	for i in (0..s.len()).rev() {
		reversed_s.push(s.as_bytes()[i] as char);
	}

	// If s is palindrome return true
	if s == reversed_s {
		return true;
	}

	// Otherwise false
	false
}

fn main() {

	// Vector of palindromes numbers higher than 100 and lower than 999
	let mut higher_palindrome: u32 = 0;

	// Try all the combinations
	// For each number from 100 to 999 (factor i)
	for i in 100..=999 {

		// For each number from 100 to 999 (factor j)
		for j in 100..=999 {

			// If the result is palindrome, add it to palindromes
			if is_palindrome((i * j).to_string()) && i * j > higher_palindrome {
				higher_palindrome = i * j;
			}
		}
	}

	println!("{}", higher_palindrome);
}