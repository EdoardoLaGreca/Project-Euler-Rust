/*
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
*/

extern crate num;

use num::BigUint;

fn calc_pow(num: u128, exp: u32) -> BigUint {

    // Init the result
    let mut result: BigUint = BigUint::new(vec![1]);

    // Calculate power
    for _ in 0..exp {
        result *= num
    }

    // Return result
    return result
}

fn get_digits(number: BigUint) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    let number_str = number.to_str_radix(10);

    for digit in number_str.chars() {
        result.push(digit.to_digit(10).unwrap() as u8);
    }


    return result;
}

fn main() {

    // num^(exp)
    let num = 2;
    let exp = 1000;

    let pow = calc_pow(num, exp);

    let digits = get_digits(pow);

    let mut result: u64 = 0;

    // Sum all the digits
    for digit in digits {
        result += digit as u64;
    }

    println!("{}", result);

}
