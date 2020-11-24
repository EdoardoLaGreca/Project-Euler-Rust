/*
If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there
are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
letters would be used?


NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
numbers is in compliance with British usage.
*/

// NOTE: Code is not optimized in terms of resources and performance. I just wrote it
//       as much understandable as possible.

// Transcribes the number into letters and returns a vector made of items grouped
// by decimal position.
fn number_transcription(num: u32) -> Vec<String> {

    // The result as a vector of words
    let mut result_vec: Vec<String> = vec![];

    let num_str = num.to_string();

    // Must be initialized
    let mut previous_digit: char = '0';

    // Position of "and" word (counting from right), it's not always the same
    let mut and_pos = 0;

    for (indx, digit) in num_str.chars().rev().enumerate() {

        // Determine decimal position using indx (usize) and the digit value
        // using digit (char).
        // NOTE: The match statements are NOT optimized.
        let word_to_be_inserted: &str = {
            match indx + 1 {
                1 => { // Units
                    match digit {
                        '1' => "one",
                        '2' => "two",
                        '3' => "three",
                        '4' => "four",
                        '5' => "five",
                        '6' => "six",
                        '7' => "seven",
                        '8' => "eight",
                        '9' => "nine",
                        _ => ""
                    }
                 },
                2 => { // Tens
                    match digit {
                        '1' => {
                            // Reset result_vec and insert a new word since words from
                            // 11 to 19 are sort of "irregular".
                            result_vec.clear();

                            // Check the previous digit
                            match previous_digit {
                                '0' => "ten",
                                '1' => "eleven",
                                '2' => "twelve",
                                '3' => "thirteen",
                                '4' => "fourteen",
                                '5' => "fifteen",
                                '6' => "sixteen",
                                '7' => "seventeen",
                                '8' => "eighteen",
                                '9' => "nineteen",
                                _ => ""
                            }
                        },
                        '2' => "twenty",
                        '3' => "thirty",
                        '4' => "forty",
                        '5' => "fifty",
                        '6' => "sixty",
                        '7' => "seventy",
                        '8' => "eighty",
                        '9' => "ninety",
                        _ => ""
                    }
                },
                3 => { // Hundreds
                    match digit {
                        '1' => "one hundred",
                        '2' => "two hundred",
                        '3' => "three hundred",
                        '4' => "four hundred",
                        '5' => "five hundred",
                        '6' => "six hundred",
                        '7' => "seven hundred",
                        '8' => "eight hundred",
                        '9' => "nine hundred",
                        _ => ""
                    }
                },
                4 => { // Thousands
                    match digit {
                        '1' => "one thousand",
                        '2' => "two thousand",
                        '3' => "three thousand",
                        '4' => "four thousand",
                        '5' => "five thousand",
                        '6' => "six thousand",
                        '7' => "seven thousand",
                        '8' => "eight thousand",
                        '9' => "nine thousand",
                        _ => ""
                    }
                }
                _ => ""
            }
        };

        result_vec.insert(0, word_to_be_inserted.to_string());

        // Remember the "and" word position
        if indx == 2 {
            and_pos = result_vec.len() - 1;
        }

        previous_digit = digit;


    }

    let rvec_len = result_vec.len();

    // Add "and" word if needed
    if num_str.len() > 2 {

        // Check if at least one of the last two items of a vector is not empty
        let are_last_two_items_empty = result_vec[rvec_len - 1] != String::new() || result_vec[rvec_len - 2] != String::new();

        if are_last_two_items_empty {
            result_vec.insert(rvec_len - and_pos, "and".to_string());
        }
    }

    return result_vec;

}

// Do not consider spaces and hyphens
fn count_chars(str_vec: Vec<String>) -> u32 {
    let mut characters: u32 = 0;

    for str in str_vec {
        for character in str.chars() {
            if character != ' ' && character != '-' {
                characters += 1_u32;
            }
        }
    }

    characters
}

// Return Some(n) if there should be a hyphen at position n. None if there shouldn't.
fn put_hyphen(vec: Vec<String>) -> Option<usize> {
    // vec's length
    let v_len = vec.len();

    let put_it: bool;

    // Put hyphen if needed.
    // To put hyphen, there must be at least 2 items in the vector.
    if v_len >= 2 {

        // Check the last two items of vec.
        // None of the last two items of vec can be an empty string, this variable is true
        // where there is at least one empty item between the last two.
        let check_last_2_items_of_vec: bool = vec[v_len - 1] == String::new() || vec[v_len - 2] == String::new();

        // Check if one of the last two items is "and", true if so.
        let one_of_them_is_and: bool = vec[v_len - 1] == "and".to_string() || vec[v_len - 2] == "and".to_string();

        if !check_last_2_items_of_vec && !one_of_them_is_and {
            put_it = true;
        } else {
            put_it = false;
        }

    } else {
        put_it = false;
    }

    if put_it {
        return Some(v_len - 2);
    } else {
        return None;
    }
}

// Build a string from that vector of strings
fn prettify(vec: Vec<String>) -> String {
    let result: String = {
        let mut res = String::new();

        let hyphen_position: Option<usize> = put_hyphen(vec.clone());

        for (i, word) in vec.clone().iter().enumerate() {

            res += word;

            if let Some(n) = hyphen_position {
                if i == n {
                    res += "-";
                }
            }

            // Last character
            let last_char = res.chars().nth(res.len() - 1).unwrap();

            // Don't add space if the previous character was a hyphen or a space
            if last_char != '-' && last_char != ' '  {
                res += " ";
            }
        }

        // Pop the last character (can be either space or useless hyphen)
        res.pop();

        res
    };

    return result;
}

fn main() {

    let mut total_n_chars = 0;

    for number in 1..=1000 {
        let result = number_transcription(number);

        let len = count_chars(result.clone());

        // Would waste too much time
        // println!("{}", prettify(result.clone()));

        // println!("{:?}, len: {}", result, len); //DEBUG

        total_n_chars += len;
    }

    println!("{}", total_n_chars);
}
