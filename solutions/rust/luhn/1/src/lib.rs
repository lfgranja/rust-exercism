/// Validates a string as a Luhn code.
pub fn is_valid(code: &str) -> bool {
    let filtered_code: String = code.chars().filter(|c| *c != ' ').collect();
    if filtered_code.len() <= 1 {
        return false;
    }

    filtered_code
        .chars()
        .rev()
        .try_fold((0, 0), |(count, sum), character| {
            character.to_digit(10).map(|number| {
                if count % 2 == 1 {
                    let doubled = number * 2;
                    (
                        count + 1,
                        sum + if doubled > 9 { doubled - 9 } else { doubled },
                    )
                } else {
                    (count + 1, sum + number)
                }
            })
        })
        .map_or(false, |(_, sum)| sum % 10 == 0)
}
