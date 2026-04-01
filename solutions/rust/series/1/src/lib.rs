//! rust/series/src/lib.rs

/*
Instructions
Given a string of digits, output all the contiguous substrings of length n in that string in the order that they appear.

For example, the string "49142" has the following 3-digit series:

"491"
"914"
"142"
And the following 4-digit series:

"4914"
"9142"
And if you ask for a 6-digit series from a 5-digit string, you deserve whatever you get.

Note that these series are only required to occupy adjacent positions in the input; the digits need not be numerically consecutive.

Different languages on Exercism have different expectations about what the result should be if the length of the substrings is zero. On the Rust track, we don't have a test for that case, so you are free to do what you feel is most appropriate.

Consider the advantages and disadvantages of the following possibilities:

Crash the program with panic!.
Return a Result::Err. (not possible here, because the function signature is given)
Return an empty vector.
Return a vector containing as many empty strings as the length of the string "digits" plus one. (this has some nice mathematical properties!)

 */

use std::str::from_utf8;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); digits.len() + 1],
        _ => digits
            .as_bytes()
            .windows(len)
            .map(|w| from_utf8(w).unwrap().to_string())
            .collect(),
    }
}
