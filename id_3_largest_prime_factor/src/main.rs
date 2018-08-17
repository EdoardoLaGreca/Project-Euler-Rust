/*
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
*/

// Check if a number is prime
fn is_prime(n: i64) -> bool {

	if n == 0 || n == 1 {
		return false;
	} else if n == 2 {
		return true;
	}

	// For each number between 1 and n/2
	for i in 3..=(n/2) {

		// If n module i is 0 return false
		if n % i == 0 && n != i && i != 1 {
			return false;
		}
	}

	// If the for loop breaks without returning false, is prime
	return true;
}

// Check divisibility among numbers
fn check_div(n: i128, divisor: u64) -> bool {

	// No number can be divided by 0
	if divisor == 0 {
		return false;
	}

	return n % (divisor as i128) == 0;
}

fn main() {

/*

About the way I used to determine the prime factors:
Let's take for example the number 210.
Now let's draw a vertical line and put the number I chose in the left column

 210 |
     |
     |
     |
     |

Now divide it by the lowest prime number greater than 1 which is a multiple of 210,
this number is 2:

 210 | 2
 105 |
     |
     |
     |

We have 105, which is 210 divided by 2. Repeat the previous step but with one exception: 2 is no
longer the multiple of the number we have now (105) so we will change it to 3, which is the nearest
prime greater than 2.

 210 | 2
 105 | 3
  35 |
     |
     |

Again: by dividing 105 by 3, we get 35. 35 is no longer a multiple of 3 so we change it to 5, which
is the nearest prime greater than 2.

 210 | 2
 105 | 3
  35 | 5
   7 |
     |

Now we got 7 and since 7 is a prime number, we will divide it by itself, in this way we will get 1.

 210 | 2
 105 | 3
  35 | 5
   7 | 7
   1 |

When we get 1, the steps are over and we can read the result: 210 is equal to 2 * 3 * 5 * 7.
Obviously we don't need 1 since the result wouldn't change.

Note that the numbers in the right column can be repeated, for example 12:

 12 | 2
  6 | 2
  3 | 3
  1 |

12 = 2 * 2 * 3

*/
	let mut num: u64 = 600851475143;

	// The divisor is explained above
	let mut divisor = 2;

	// Prime factors of num
	let mut prime_factors_of_num: Vec<u64> = vec![];

	println!("num = {:?}, divisor = {:?}, prime_factors_of_num = {:?}", num, divisor, prime_factors_of_num);

	// Get prime factors of num
	while !is_prime(num as i64) {

		// If divisor is a multiple of num, divide this by divisor and add the
		// divisor to prime_factors_of_num
		if check_div(num as i128, divisor) {
			num /= divisor;
			prime_factors_of_num.push(divisor);
		} else {
			divisor += 1;

			// Find next divisor
			while !is_prime(divisor as i64) {
				divisor += 1;
			}
		}

		// Executed on the last iteration of the loop
		if is_prime(num as i64) {
			prime_factors_of_num.push(num);
		}

		println!("num = {:?}, divisor = {:?}, prime_factors_of_num = {:?}", num, divisor, prime_factors_of_num);

	}

	// Find the largest prime
	// The largest prime, in this way, will always be the last
	let largest_prime = prime_factors_of_num[prime_factors_of_num.len() - 1];

	println!("{}", largest_prime);

}
