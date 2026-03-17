use anagram::anagrams_for;

fn main() {
    let word = "stone";
    let possible_anagrams = &["stone", "tones", "banana", "tons", "notes", "Seton"];
    let anagrams = anagrams_for(word, possible_anagrams);
    println!("Anagrams for '{word}': {anagrams:?}");
}
