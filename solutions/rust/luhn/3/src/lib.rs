/*
 * luhn/src/lib.rs
 */

pub fn is_valid(code: &str) -> bool {
    if code.chars().filter(|character| *character != ' ').count() <= 1 {
        return false;
    }

    code.chars()
        .filter(|character| *character != ' ')
        .rev()
        .try_fold((0, 0), |(count, sum), character| {
            character.to_digit(10).map(|number| {
                if count % 2 == 1 {
                    if number * 2 >= 10 {
                        (count + 1, sum + number * 2 - 9)
                    } else {
                        (count + 1, sum + number * 2)
                    }
                } else {
                    (count + 1, sum + number)
                }
            })
        })
        .is_some_and(|(count, sum)| count > 1 && sum % 10 == 0)
}
