use crate::aoc_2022_3::LetterField;

fn count_distinct_letters(field: &LetterField) -> usize {
    let mut f = field.0;
    let mut n = 0;
    while f != 0 {
        if f & 1 > 0 {
            n += 1
        }
        f >>= 1;
    }
    n
}

fn run_internal(input: &str, window_size: usize) -> anyhow::Result<String> {
    let mut index = None;
    for (i, w) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
    {
        if count_distinct_letters(&LetterField::from(w.iter().cloned())) == window_size {
            index = Some((i + window_size, w.to_vec()));
            break;
        }
    }
    let r = index.ok_or(anyhow::format_err!("Couldn't find any marker"))?;
    Ok(format!("{} ({:?})", r.0, r.1))
}

pub fn run(input: &str) -> anyhow::Result<String> {
    run_internal(input, 4)
}

pub fn run2(input: &str) -> anyhow::Result<String> {
    run_internal(input, 14)
}
