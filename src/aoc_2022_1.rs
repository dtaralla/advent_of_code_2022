use anyhow::bail;
use std::collections::VecDeque;

pub fn run(input: &str) -> anyhow::Result<String> {
    let mut cur_max: u64 = 0;

    let mut cur_sum: u64 = 0;
    for line in input.lines() {
        if line.is_empty() {
            if cur_sum > cur_max {
                cur_max = cur_sum;
            }

            cur_sum = 0;
        } else {
            cur_sum += line.parse::<u64>()?;
        }
    }

    Ok(if cur_sum > cur_max { cur_sum } else { cur_max }.to_string())
}

pub fn run2(input: &str) -> anyhow::Result<String> {
    let mut maxs = VecDeque::<u64>::new();

    let mut cur_sum: u64 = 0;
    for line in input.lines() {
        if line.is_empty() {
            record_elf(&mut maxs, cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<u64>()?;
        }
    }
    record_elf(&mut maxs, cur_sum);

    return if maxs.len() < 3 {
        bail!("Need at least 3 elves in input")
    } else {
        Ok(maxs.iter().sum::<u64>().to_string())
    };
}

fn record_elf(maxs: &mut VecDeque<u64>, new: u64) {
    let idx = maxs.partition_point(|&x| x < new);
    maxs.insert(idx, new);
    if maxs.len() > 3 {
        maxs.pop_front();
    }
}
