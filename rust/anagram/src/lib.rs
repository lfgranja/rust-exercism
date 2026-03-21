/*
Introduction
At a garage sale, you find a lovely vintage typewriter at a bargain price! Excitedly, you rush home, insert a sheet of paper, and start typing away. However, your excitement wanes when you examine the output: all words are garbled! For example, it prints "stop" instead of "post" and "least" instead of "stale." Carefully, you try again, but now it prints "spot" and "slate." After some experimentation, you find there is a random delay before each letter is printed, which messes up the order. You now understand why they sold it for so little money!

You realize this quirk allows you to generate anagrams, which are words formed by rearranging the letters of another word. Pleased with your finding, you spend the rest of the day generating hundreds of anagrams.

Instructions
Given a target word and one or more candidate words, your task is to find the candidates that are anagrams of the target.

An anagram is a rearrangement of letters to form a new word: for example "owns" is an anagram of "snow". A word is not its own anagram: for example, "stop" is not an anagram of "stop".

The target word and candidate words are made up of one or more ASCII alphabetic characters (A-Z and a-z). Lowercase and uppercase characters are equivalent: for example, "PoTS" is an anagram of "sTOp", but "StoP" is not an anagram of "sTOp". The words you need to find should be taken from the candidate words, using the same letter case.

Given the target "stone" and the candidate words "stone", "tones", "banana", "tons", "notes", and "Seton", the anagram words you need to find are "tones", "notes", and "Seton".

The Rust track extends the possible letters to be any unicode character, not just ASCII alphabetic ones.

You are going to have to adjust the function signature provided in the stub in order for the lifetimes to work out properly. This is intentional: what's there demonstrates the basics of lifetime syntax, and what's missing teaches how to interpret lifetime-related compiler errors.
*/

use std::collections::HashSet;

///
/// Finds all anagrams of `word` in the given list of `possible_anagrams`.
///
/// # Examples
///
/// ```
/// let anagrams = anagrams_for("stone", &["stone", "tones", "banana", "tons", "notes", "Seton"]);
/// assert_eq!(anagrams, HashSet::from(["tones", "notes", "Seton"]));
/// ```
///
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let mut sorted_chars: Vec<char> = word_lowercase.chars().collect();
    sorted_chars.sort_unstable();

    possible_anagrams
        .iter()
        .copied()
        .filter(|&candidate| {
            let candidate_lowercase = candidate.to_lowercase();
            if candidate_lowercase.len() != word_lowercase.len() {
                return false;
            }
            if candidate_lowercase == word_lowercase {
                return false;
            }
            let mut candidate_chars: Vec<char> = candidate_lowercase.chars().collect();
            candidate_chars.sort_unstable();
            candidate_chars == sorted_chars
        })
        .collect()
}
