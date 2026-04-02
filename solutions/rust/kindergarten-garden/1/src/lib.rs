pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let start_index = ((student.as_bytes()[0] - b'A') * 2) as usize;

    diagram
        .lines()
        .flat_map(|line| {
            line.as_bytes()[start_index..start_index + 2]
                .iter()
                .map(|&byte| match byte {
                    b'G' => "grass",
                    b'C' => "clover",
                    b'R' => "radishes",
                    b'V' => "violets",
                    _ => unreachable!(),
                })
        })
        .collect()
}
