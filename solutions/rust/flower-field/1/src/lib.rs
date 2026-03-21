pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let rows = garden.len();
    let columns = garden[0].len();

    garden
        .iter()
        .enumerate()
        .map(|(r, row)| {
            let mut new_row = Vec::with_capacity(columns);
            let row_bytes = row.as_bytes();

            for c in 0..columns {
                if row_bytes[c] == b'*' {
                    new_row.push(b'*');
                } else {
                    let mut count = 0;

                    let r_start = r.saturating_sub(1);
                    let r_end = (r + 1).min(rows - 1);
                    let c_start = c.saturating_sub(1);
                    let c_end = (c + 1).min(columns - 1);

                    for nr in r_start..=r_end {
                        for nc in c_start..=c_end {
                            if garden[nr].as_bytes()[nc] == b'*' {
                                count += 1;
                            }
                        }
                    }

                    if count == 0 {
                        new_row.push(b' ');
                    } else {
                        new_row.push(b'0' + count as u8);
                    }
                }
            }
            String::from_utf8(new_row).expect("Done")
        })
        .collect()
}
