#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Score {
    Loss,
    Draw,
    Win,
}

impl From<char> for Score {
    fn from(c: char) -> Self {
        match c {
            'X' => Score::Loss,
            'Y' => Score::Draw,
            'Z' => Score::Win,
            unsupported => panic!("Unknown score: {}", unsupported),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn beats_what(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    pub fn beaten_by(&self) -> Shape {
        match self {
            Shape::Scissors => Shape::Rock,
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
        }
    }

    pub fn beats(&self, other: &Shape) -> bool {
        self.beats_what() == *other
    }

    pub fn against(&self, other: &Shape) -> Score {
        match (self, other) {
            (mine, theirs) if mine.beats(theirs) => Score::Win,
            (mine, theirs) if theirs.beats(mine) => Score::Loss,
            _ => Score::Draw,
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            unsupported => panic!("Unknown shape: {}", unsupported),
        }
    }
}

impl From<Shape> for u32 {
    fn from(s: Shape) -> Self {
        s.points()
    }
}

impl From<Score> for u32 {
    fn from(s: Score) -> Self {
        match s {
            Score::Loss => 0,
            Score::Draw => 3,
            Score::Win => 6,
        }
    }
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let mut total_score: u32 = 0;

    for line in input.lines() {
        assert!(line.len() >= 3);
        let opponent_shape = Shape::from(line.chars().nth(0).unwrap());
        let my_shape = Shape::from(line.chars().nth(2).unwrap());

        let score = my_shape.against(&opponent_shape);
        total_score = total_score + u32::from(my_shape) + u32::from(score);
    }

    Ok(total_score.to_string())
}

pub fn run2(input: &str) -> anyhow::Result<String> {
    let mut total_score: u32 = 0;

    for line in input.lines() {
        assert!(line.len() >= 3);
        let opponent_shape = Shape::from(line.chars().nth(0).unwrap());
        let target_score = Score::from(line.chars().nth(2).unwrap());

        let my_shape = match target_score {
            Score::Loss => opponent_shape.beats_what(),
            Score::Draw => opponent_shape,
            Score::Win => opponent_shape.beaten_by(),
        };
        let score = my_shape.against(&opponent_shape);
        total_score = total_score + u32::from(my_shape) + u32::from(score);
    }

    Ok(total_score.to_string())
}
