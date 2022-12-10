struct Range<T: Ord + Copy> {
    min: T,
    max: T,
}

impl<T: Ord + Copy> Range<T> {
    fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    fn overlaps(&self, other: &Self) -> bool {
        !(self.min > other.max || self.max < other.min)
    }
}

impl From<&str> for Range<usize> {
    fn from(value: &str) -> Self {
        let mut vs = value.split('-');
        Self {
            min: vs.next().unwrap().parse::<usize>().unwrap(),
            max: vs.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let mut count = 0;
    for line in input.lines() {
        let mut ranges = line.split(',');
        let range1 = Range::from(ranges.next().unwrap());
        let range2 = Range::from(ranges.next().unwrap());

        if range1.contains(&range2) || range2.contains(&range1) {
            count += 1;
        }
    }
    Ok(count.to_string())
}

pub fn run2(input: &str) -> anyhow::Result<String> {
    let mut count = 0;
    for line in input.lines() {
        let mut ranges = line.split(',');
        let range1 = Range::from(ranges.next().unwrap());
        let range2 = Range::from(ranges.next().unwrap());

        if range1.overlaps(&range2) {
            count += 1;
        }
    }
    Ok(count.to_string())
}
