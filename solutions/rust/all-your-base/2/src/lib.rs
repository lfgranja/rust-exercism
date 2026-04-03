use std::iter::successors;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match (from_base, to_base) {
        (0..=1, _) => Err(Error::InvalidInputBase),
        (_, 0..=1) => Err(Error::InvalidOutputBase),
        _ => number
            .iter()
            .try_fold(0, |acc, &d| {
                (d < from_base)
                    .then_some(acc * from_base + d)
                    .ok_or(Error::InvalidDigit(d))
            })
            .map(|base_10_val| {
                let mut digits = successors(Some(base_10_val), |&n| {
                    (n >= to_base).then_some(n / to_base)
                })
                .map(|n| n % to_base)
                .collect::<Vec<_>>();
                digits.reverse();
                digits
            }),
    }
}
