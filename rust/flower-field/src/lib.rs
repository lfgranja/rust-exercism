/*
 *
 * flower-field/src/lib.rs
 *
Introduction
Flower Field is a compassionate reimagining of the popular game Minesweeper. The object of the game is to find all the flowers in the garden using numeric hints that indicate how many flowers are directly adjacent (horizontally, vertically, diagonally) to a square. "Flower Field" shipped in regional versions of Microsoft Windows in Italy, Germany, South Korea, Japan and Taiwan.

Instructions
Your task is to add flower counts to empty squares in a completed Flower Field garden. The garden itself is a rectangle board composed of squares that are either empty (' ') or a flower ('*').

For each empty square, count the number of flowers adjacent to it (horizontally, vertically, diagonally). If the empty square has no adjacent flowers, leave it empty. Otherwise replace it with the count of adjacent flowers.

For example, you may receive a 5 x 4 board like this (empty spaces are represented here with the '·' character for display on screen):

·*·*·
··*··
··*··
·····
Which your code should transform into this:

1*3*1
13*31
·2*2·
·111·
Performance Hint
All the inputs and outputs are in ASCII. Rust Strings and &str are utf8, so while one might expect "Hello".chars() to be simple, it actually has to check each character to see if it's 1, 2, 3 or 4 u8s long. If we know a &str is ASCII then we can call .as_bytes() and refer to the underlying data as a &[u8] (byte slice). Iterating over a slice of ASCII bytes is much quicker as there are no codepoints involved - every ASCII byte is one u8 long.

Can you complete the challenge without cloning the input?
*/

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row_index, row_content)| {
            // We use .as_bytes() for performance since the input is guaranteed to be ASCII.
            // This avoids the overhead of UTF-8 character decoding.
            row_content
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(column_index, &cell_content)| match cell_content {
                    b'*' => '*',
                    _ => {
                        // Generate a 3x3 window around the current cell.
                        // .saturating_sub(1) prevents underflow if index is 0.
                        // row_index + 1 might go out of bounds, but garden.get() handles this safely.
                        match (row_index.saturating_sub(1)..=row_index + 1)
                            .flat_map(|adjacent_row| {
                                (column_index.saturating_sub(1)..=column_index + 1).filter_map(
                                    move |adjacent_column| {
                                        // garden.get() returns None if row is out of bounds.
                                        // .as_bytes().get() returns None if column is out of bounds.
                                        // The '?' operator (via filter_map) skips these None values.
                                        garden.get(adjacent_row)?.as_bytes().get(adjacent_column)
                                    },
                                )
                            })
                            // Count only the bytes that represent a flower ('*').
                            .filter(|&adjacent_byte| *adjacent_byte == b'*')
                            .count()
                        {
                            // Per requirements: no adjacent flowers means an empty space.
                            0 => ' ',
                            // Convert the numeric count (0-8) to its ASCII character equivalent ('1'-'8').
                            flower_count => (b'0' + flower_count as u8) as char,
                        }
                    }
                })
                .collect::<String>()
        })
        .collect()
}
