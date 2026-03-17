use std::collections::HashSet;

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
