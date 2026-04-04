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
