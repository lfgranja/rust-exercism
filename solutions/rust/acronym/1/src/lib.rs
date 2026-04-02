/*
Instructions
Convert a phrase to its acronym.

Techies love their TLA (Three Letter Acronyms)!

Help generate some jargon by writing a program that converts a long name like Portable Network Graphics to its acronym (PNG).

Punctuation is handled as follows: hyphens are word separators (like whitespace); all other punctuation can be removed from the input.

For example:

Input	Output
As Soon As Possible	ASAP
Liquid-crystal display	LCD
Thank George It's Friday!	TGIF
 */

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .filter(|&c| c != '\'')
        .fold((String::new(), ' '), |(mut acc, prev), curr| {
            let is_boundary = !prev.is_alphabetic();
            let is_camel_case = prev.is_lowercase() && curr.is_uppercase();
            if curr.is_alphabetic() && (is_boundary || is_camel_case) {
                acc.extend(curr.to_uppercase())
            }
            (acc, curr)
        })
        .0
}
