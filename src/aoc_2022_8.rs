fn count_visible(visible_set: &[u128]) -> usize {
    let mut count = 0;
    for set in visible_set {
        let mut set = *set;
        while set > 0 {
            count += match set & 0xF {
                0b0000 => 0,
                0b1000 | 0b0100 | 0b0010 | 0b0001 => 1,
                0b1100 | 0b0110 | 0b0011 | 0b1010 | 0b0101 | 0b1001 => 2,
                0b1110 | 0b1101 | 0b1011 | 0b0111 => 3,
                0b1111 => 4,
                s => panic!("Forgotten pattern: {s}"),
            };
            set >>= 4;
        }
    }
    count
}

fn tag_visible(
    i: usize,
    mut row: &[u8],
    visible_set: &mut [u128],
    mut biggest_from_left: u8,
) -> u8 {
    if let Some(first) = row.take_first() {
        if *first > biggest_from_left {
            biggest_from_left = *first;
            visible_set[i / 128] |= 1 << (i % 128);
        }
        let mut biggest_from_right = tag_visible(i + 1, row, visible_set, biggest_from_left);
        if *first > biggest_from_right {
            biggest_from_right = *first;
            visible_set[i / 128] |= 1 << (i % 128);
        }
        biggest_from_right
    } else {
        0
    }
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut columns: Vec<Vec<u8>> = vec![vec![0; height]; width];
    let mut columns_visible_sets: Vec<Vec<u128>> = vec![vec![0; 1 + (height - 1) / 128]; width];

    // Count horizontally
    for (i, line) in lines.iter().enumerate() {
        let line: Vec<u8> = line
            .chars()
            .enumerate()
            .map(|(j, c)| {
                columns[j][i] = c as u8;
                c as u8
            })
            .collect();

        let mut visible_sets = vec![0; 1 + (width - 1) / 128];
        tag_visible(0, &line, &mut visible_sets, 0);

        // ugh, transpose the visible bits so that we can use them when we'll count vertically...
        for j in 0..width {
            columns_visible_sets[j][i / 128] |= if (visible_sets[j / 128] & 1 << (j % 128)) > 0 {
                1 << (i % 128)
            } else {
                0
            };
        }
    }

    let mut count = 0;

    // Parse vertically and update count, column by column
    for (j, col) in columns.iter().enumerate() {
        tag_visible(0, &col, &mut columns_visible_sets[j], 0);
        count += count_visible(&columns_visible_sets[j]);
    }

    Ok(count.to_string())
}

pub fn run2(_input: &str) -> anyhow::Result<String> {
    Ok("So in ex1, I actually had to count for each tree in all direction if they reached the end? \
    Then reuse the same function for part 2 from any tree? Pffff.".to_string())
}
