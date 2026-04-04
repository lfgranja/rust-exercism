//! rust/alphametics/src/lib.rs

/*
Instructions
Given an alphametics puzzle, find the correct solution.

Alphametics is a puzzle where letters in words are replaced with numbers.

For example SEND + MORE = MONEY:

  S E N D
  M O R E +
-----------
M O N E Y
Replacing these with valid numbers gives:

  9 5 6 7
  1 0 8 5 +
-----------
1 0 6 5 2
This is correct because every letter is replaced by a different number and the words, translated into numbers, then make a valid sum.

Each letter must represent a different digit, and the leading digit of a multi-digit number must not be zero.


 */

use std::collections::{HashMap, HashSet};
use std::iter::once;

/// Solves an alphametics puzzle and returns a mapping of letters to digits.
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once(" == ")?;

    let (weights, non_zeroes) = left
        .split(" + ")
        .map(|w| (w, 1i64))
        .chain(once((right, -1i64)))
        .fold((HashMap::new(), HashSet::new()), |mut acc, (word, sign)| {
            if word.len() > 1 {
                acc.1.insert(word.chars().next().unwrap());
            }
            word.chars()
                .rev()
                .enumerate()
                .for_each(|(i, c)| *acc.0.entry(c).or_insert(0) += sign * 10_i64.pow(i as u32));
            acc
        });

    let mut letters: Vec<(char, i64, bool)> = weights
        .into_iter()
        .map(|(c, w)| (c, w, non_zeroes.contains(&c)))
        .collect();
    letters.sort_unstable_by_key(|&(_, weights, _)| -weights.abs());

    let mut solution = vec![0; letters.len()];
    backtrack(0, 0, 0, &letters, &mut solution).then(|| {
        letters
            .into_iter()
            .zip(solution)
            .map(|((c, _, _), d)| (c, d))
            .collect::<HashMap<char, u8>>()
    })
}

/// Backtracking function to solve the alphametics puzzle.
///
/// `idx` is the current index in `letters`,
/// `current_sum` is the current sum,
/// `used_digits` is a bitmask of used digits,
/// `letters` is the vector of letter data,
/// `solution` is the vector of digit solutions.
///
/// Returns `true` if a solution is found, `false` otherwise.
fn backtrack(
    idx: usize,
    current_sum: i64,
    used_digits: u16,
    letters: &[(char, i64, bool)],
    solution: &mut [u8],
) -> bool {
    if idx == letters.len() {
        return current_sum == 0;
    }
    let digit_value = letters[idx].1;
    let cannot_be_zero = letters[idx].2;
    for digit in 0..=9 {
        if used_digits & (1 << digit) != 0 || digit == 0 && cannot_be_zero {
            continue;
        }

        solution[idx] = digit as u8;

        if backtrack(
            idx + 1,
            current_sum + digit_value * digit as i64,
            used_digits | (1 << digit),
            letters,
            solution,
        ) {
            return true;
        }
    }
    false
}
