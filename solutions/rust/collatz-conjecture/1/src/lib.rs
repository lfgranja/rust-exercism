pub fn collatz(n: u64) -> Option<u64> {
    (0_u64..)
        .try_fold(n, |current, step| match current {
            0 => Err(None), // If 0 is the input, return None immediately, short-circuiting the loop
            1 => Err(Some(step)), // If 1 is reached, return the step count as an Err to break out of the loop
            x if x % 2 == 0 => Ok(x / 2), // If even, divide by 2 inside an Ok to continue the loop
            x => Ok(x
                .checked_mul(3)
                .and_then(|x| x.checked_add(1))
                .ok_or(None)?), // If odd, multiply safely by 3, add 1, and continue inside an Ok. The "?" operator propagates the error if multiplication or addition fails.
        })
        .err() // As we know the only way out of our infinite range loop is by means of an Err (either 0, reaching 1 or overflowing), we can safely convert the Result into an Option<Option<u64>>.
        .flatten() // Flattens the Option<Option<u64>> into Option<u64>, containing the number of steps or None if 0 or overflow occurred.
}
