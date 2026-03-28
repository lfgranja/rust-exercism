use std::iter::once;

pub fn nth(n: u32) -> u32 {
    once(2)
        .chain((3u32..).step_by(2))
        .filter(|&candidate| {
            !(3..=candidate.isqrt())
                .step_by(2)
                .any(|d| candidate % d == 0)
        })
        .nth(n as usize)
        .unwrap()
}
