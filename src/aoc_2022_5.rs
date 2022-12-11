use anyhow::bail;
use lazy_static::lazy_static;
use regex::Regex;
use smallvec::{smallvec, SmallVec};
use std::fmt::{Display, Formatter, Write};

#[derive(Default)]
struct CrateStack(SmallVec<[char; 512]>);

impl CrateStack {
    fn move_items_to(
        &mut self,
        n_items: usize,
        other: &mut Self,
        one_by_one: bool,
    ) -> anyhow::Result<()> {
        let len = self.0.len();
        let new_len = len - n_items;
        let tail = self
            .0
            .as_slice()
            .take(new_len..)
            .ok_or(anyhow::format_err!(
                "Stack has {} items; can't take {n_items} from it",
                len
            ))?;

        if one_by_one {
            other.0.extend(tail.iter().cloned().rev());
        } else {
            other.0.extend_from_slice(tail);
        }

        self.0.truncate(new_len);
        Ok(())
    }
}

struct Situation {
    stacks: SmallVec<[CrateStack; 16]>,
}

impl Situation {
    pub fn execute(&mut self, m: Move, one_by_one: bool) -> anyhow::Result<()> {
        if m.to == m.from {
            bail!("Can't move to the same pile!");
        }

        let [from, to] = self.stacks.get_many_mut([m.from - 1, m.to - 1])?;
        from.move_items_to(m.n_items, to, one_by_one)
    }
}

impl Display for Situation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_length = self.stacks.iter().map(|stack| stack.0.len()).max().unwrap();
        for i in (0..max_length).rev() {
            self.stacks.iter().map(|stack| &stack.0).for_each(|stack| {
                if let Some(c) = stack.get(i) {
                    write!(f, "[{c}] ").unwrap();
                } else {
                    write!(f, "    ").unwrap();
                }
            });
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<'a, T: Iterator<Item = &'a str>> From<T> for Situation {
    fn from(mut lines: T) -> Self {
        let n_stacks = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut stacks = smallvec![];
        stacks.resize_with(n_stacks, Default::default);

        for stack_line in lines {
            stack_line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if i % 4 == 1 { Some(c) } else { None })
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| (&mut stacks[i] as &mut CrateStack).0.push(c));
        }

        Self { stacks }
    }
}

struct Move {
    n_items: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let matches = RE.captures(value).unwrap();
        Self {
            n_items: matches.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            from: matches.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            to: matches.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }
}

fn run_internal(input: &str, one_by_one: bool) -> anyhow::Result<String> {
    let mut line_iter = input.lines();

    let config_input = line_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .into_iter()
        .rev();

    let mut config = Situation::from(config_input);

    for line in line_iter {
        let m = Move::from(line);
        config.execute(m, one_by_one)?;
    }

    let mut result = String::with_capacity(config.stacks.len());
    config
        .stacks
        .iter()
        .map(|s| s.0.last().unwrap())
        .for_each(|c| write!(&mut result, "{c}").unwrap());

    Ok(result)
}

pub fn run(input: &str) -> anyhow::Result<String> {
    run_internal(input, true)
}

pub fn run2(input: &str) -> anyhow::Result<String> {
    run_internal(input, false)
}
