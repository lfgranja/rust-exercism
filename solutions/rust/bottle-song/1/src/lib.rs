const NUMBERS_IN_LOWERCASE: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

const NUMBERS_IN_CAPITALCASE: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|current_step| start_bottles - current_step)
        .map(|current_bottles| {
            let next_bottles = current_bottles - 1;

            let s = |n| if n == 1 { "" } else { "s" };

            format!(
                "{current_bottles_capital} green bottle{} hanging on the wall,\n\
                 {current_bottles_capital} green bottle{} hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be {next_bottles_lower} green bottle{} hanging on the wall.",
                s(current_bottles),
                s(current_bottles),
                s(next_bottles),
                current_bottles_capital = NUMBERS_IN_CAPITALCASE[current_bottles as usize],
                next_bottles_lower = NUMBERS_IN_LOWERCASE[next_bottles as usize],
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
