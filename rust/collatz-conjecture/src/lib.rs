//! rust/collatz-conjecture/src/lib.rs

/*
Introduction
One evening, you stumbled upon an old notebook filled with cryptic scribbles, as though someone had been obsessively chasing an idea. On one page, a single question stood out: Can every number find its way to 1? It was tied to something called the Collatz Conjecture, a puzzle that has baffled thinkers for decades.

The rules were deceptively simple. Pick any positive integer.

If it's even, divide it by 2.
If it's odd, multiply it by 3 and add 1.
Then, repeat these steps with the result, continuing indefinitely.

Curious, you picked number 12 to test and began the journey:

12 ➜ 6 ➜ 3 ➜ 10 ➜ 5 ➜ 16 ➜ 8 ➜ 4 ➜ 2 ➜ 1

Counting from the second number (6), it took 9 steps to reach 1, and each time the rules repeated, the number kept changing. At first, the sequence seemed unpredictable — jumping up, down, and all over. Yet, the conjecture claims that no matter the starting number, we'll always end at 1.

It was fascinating, but also puzzling. Why does this always seem to work? Could there be a number where the process breaks down, looping forever or escaping into infinity? The notebook suggested solving this could reveal something profound — and with it, fame, fortune, and a place in history awaits whoever could unlock its secrets.

Instructions
Given a positive integer, return the number of steps it takes to reach 1 according to the rules of the Collatz Conjecture.
*/

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
