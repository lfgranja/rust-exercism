pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row_index, row_content)| {
            row_content
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(column_index, &cell_content)| match cell_content {
                    b'*' => '*',
                    _ => {
                        match (row_index.saturating_sub(1)..=row_index + 1)
                            .flat_map(|adjacent_row| {
                                (column_index.saturating_sub(1)..=column_index + 1).filter_map(
                                    move |adjacent_column| {
                                        garden.get(adjacent_row)?.as_bytes().get(adjacent_column)
                                    },
                                )
                            })
                            .filter(|&adjacent_byte| *adjacent_byte == b'*')
                            .count()
                        {
                            0 => ' ',
                            flower_count => (b'0' + flower_count as u8) as char,
                        }
                    }
                })
                .collect::<String>()
        })
        .collect()
}