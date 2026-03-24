/*
 * luhn/src/lib.rs
 */

pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|character| !character.is_ascii_whitespace())
        .rev()
        .try_fold((0, 0), |(count, sum), character| {
            character.to_digit(10).map(|number| {
                let doubled = if count % 2 == 1 { number * 2 } else { number };
                let adjusted = if doubled > 9 { doubled - 9 } else { doubled };
                (count + 1, sum + adjusted)
            })
        })
        .is_some_and(|(count, sum)| count > 1 && sum % 10 == 0)
}
