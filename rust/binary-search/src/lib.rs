//! rust/binary-search/src/lib.rs

/*
Introduction
You have stumbled upon a group of mathematicians who are also singer-songwriters. They have written a song for each of their favorite numbers, and, as you can imagine, they have a lot of favorite numbers (like 0 or 73 or 6174).

You are curious to hear the song for your favorite number, but with so many songs to wade through, finding the right song could take a while. Fortunately, they have organized their songs in a playlist sorted by the title — which is simply the number that the song is about.

You realize that you can use a binary search algorithm to quickly find a song given the title.

Instructions
Your task is to implement a binary search algorithm.

A binary search algorithm finds an item in a list by repeatedly splitting it in half, only keeping the half which contains the item we're looking for. It allows us to quickly narrow down the possible locations of our item until we find it, or until we've eliminated all possible locations.

Caution
Binary search only works when a list has been sorted.

The algorithm looks like this:

Find the middle element of a sorted list and compare it with the item we're looking for.
If the middle element is our item, then we're done!
If the middle element is greater than our item, we can eliminate that element and all the elements after it.
If the middle element is less than our item, we can eliminate that element and all the elements before it.
If every element of the list has been eliminated then the item is not in the list.
Otherwise, repeat the process on the part of the list that has not been eliminated.
Here's an example:

Let's say we're looking for the number 23 in the following sorted list: [4, 8, 12, 16, 23, 28, 32].

We start by comparing 23 with the middle element, 16.
Since 23 is greater than 16, we can eliminate the left half of the list, leaving us with [23, 28, 32].
We then compare 23 with the new middle element, 28.
Since 23 is less than 28, we can eliminate the right half of the list: [23].
We've found our item.
Restrictions
Rust provides in its standard library already a binary search function. For this exercise you should not use this function but just other basic tools instead.

For bonus points
Did you get the tests passing and the code clean? If you want to, there are some additional things you could try.

Currently your find function will probably only work for slices of numbers, but the Rust type system is flexible enough to create a find function which works on all slices which contains elements which can be ordered.
Additionally this find function can work not only on slices, but at the same time also on a Vec or an Array.
To run the bonus tests, remove the #[ignore] flag and execute the tests with the generic feature, like this:

$ cargo test --features generic
Then please share your thoughts in a comment on the submission. Did this experiment make the code better? Worse? Did you learn anything from it?

Source
Wikipedia
 */

use std::cmp::Ordering::{Equal, Greater, Less};

pub fn find<U, T>(array: U, key: T) -> Option<usize>
where
    U: AsRef<[T]>, // It doesn't matter if U is a Vec, an Array or some other type, as long as it can be viewed as a slice &[T]
    T: Ord, // T must be able to be ordered, so we can compare it using .cmp(&key) and Ordering::{Equal, Greater, Less}
{
    let slice = array.as_ref();
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        let mid = left + (right - left) / 2; // Overflow-safe midpoint calculation
        match slice[mid].cmp(&key) {
            Equal => return Some(mid),
            Greater => right = mid,
            Less => left = mid + 1,
        }
    }

    None
}
