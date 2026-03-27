use std::iter::successors;

pub fn is_armstrong_number(num: u32) -> bool {
    // Calculate the number of digits in the number using base-10 logarithm.
    // num.checked_ilog10() returns the floor of log10(num).
    // We add 1 to get the digit count. For 0, we default to 1 digit.
    let len = num.checked_ilog10().unwrap_or(0) + 1;

    // Create an iterator that yields the number and then repeatedly divides it by 10.
    // This allows us to process the number digit by digit from right to left.
    successors(Some(num), |&n| (n >= 10).then_some(n / 10))
        // Extract the last digit of the current number (n % 10), casting to u64 to avoid overflow of the u32 type on next step (pow).
        .map(|extract_last_digit| (extract_last_digit % 10) as u64)
        // Raise the extracted digit to the power of the total number of digits.
        .map(|digit_by_digit| digit_by_digit.pow(len))
        // Sum the results. We use u64 to avoid overflow during the summation.
        .sum::<u64>()
        // Compare the final sum with the original number.
        == num as u64
}
