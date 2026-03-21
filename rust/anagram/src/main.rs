use anagram::anagrams_for;

///
/// Finds all anagrams of `word` in the given list of `possible_anagrams`.
///
/// # Examples
///
/// ```
/// let anagrams = anagrams_for("stone", &["stone", "tones", "banana", "tons", "notes", "Seton"]);
/// assert_eq!(anagrams, HashSet::from(["tones", "notes", "Seton"]));
/// ```
fn main() {
    let word = "stone";
    let possible_anagrams = &[
        "stone", "tones", "banana", "tons", "notes", "Seton", "Soten", "steNo", "sonet",
    ];
    let anagrams = anagrams_for(word, possible_anagrams);
    println!("Anagrams for '{word}': {anagrams:?}");
}
