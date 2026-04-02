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
