use std::iter::successors;

pub fn egg_count(display_value: u32) -> usize {
    successors(Some(display_value).filter(|&n| n > 0), |&n| {
        let next = n & (n - 1);
        (next > 0).then_some(next)
    })
    .count()
}
