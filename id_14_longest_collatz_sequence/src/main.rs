/*
The following iterative sequence is defined for the set of positive integers:

        n → n/2 (n is even)
        n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
        13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
Although it has not been proved yet (Collatz Problem), it is thought that all starting
numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
*/

// Given a starting number, this function calculates the number
// of terms in the sequence, using the Collatz sequence's rules
fn collatz_sequence(starting_number: u32) -> u32 {
    
    // Current number (updated in every iteration
    // of the while cycle)
    let mut current_num: u64 = starting_number as u64;

    // Sequence length
    let mut seq_len: u32 = 0;

    // Keep going until the number is 1
    while current_num != 1 {

        // Generating the Collatz sequence
        if current_num % 2 == 0 { // even
            current_num /= 2;
        } else { // odd
            current_num = (current_num * 3) + 1;
        }

        // Increase sequence length
        seq_len += 1;
    }

    return seq_len;
}

fn main() {

    // Longest chain.
    // It's a tuple where the first value is the chain's length
    // and the second one is the starting number.
    let mut longest_chain: (u32, u32) = (0, 1);

    // Number must be lesser than 1 million
    for num in 1..1_000_000 {

        // Get the chain length of the current number
        let chain_len = collatz_sequence(num);

        // If the current chain is longer than the last
        // longest one, update the value and update the 
        // starting number
        if chain_len > longest_chain.0 {
            longest_chain = (chain_len, num);
        }
    }

    println!("Starting number: {}\nIts chain's length: {}", longest_chain.1, longest_chain.0);
}
