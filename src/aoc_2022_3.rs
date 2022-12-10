use std::str::Chars;

struct LetterField(u64);

impl Default for LetterField {
    fn default() -> Self {
        Self(0)
    }
}

impl LetterField {
    fn ascii_to_bitfield_index(c: &char) -> usize {
        if c.is_ascii_uppercase() {
            (*c as usize) - 65 + 26 // 26..52 for A..Z
        } else {
            (*c as usize) - 97 //  0..26 for a..z
        }
    }

    fn combine_with(&self, other: &LetterField) -> Self {
        Self(self.0 & other.0)
    }

    fn first_true_bit_index(&self) -> usize {
        let mut index: usize = 0;
        while (self.0 & (1 << index)) == 0 {
            index += 1;
        }
        index
    }
}

impl From<Chars<'_>> for LetterField {
    fn from(value: Chars) -> Self {
        let mut field = Self::default();
        for c in value {
            field.0 |= 1 << Self::ascii_to_bitfield_index(&c);
        }
        field
    }
}

#[derive(Default)]
struct Sack {
    pub cpt1: LetterField,
    pub cpt2: LetterField,
}

impl From<&str> for Sack {
    fn from(content: &str) -> Self {
        let (cpt1, cpt2) = content.split_at(content.len() / 2);
        assert_eq!(cpt1.len(), cpt2.len());

        Self {
            cpt1: LetterField::from(cpt1.chars()),
            cpt2: LetterField::from(cpt2.chars()),
        }
    }
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let mut sum: usize = 0;

    for line in input.lines() {
        let sack = Sack::from(line);
        let result = sack.cpt1.combine_with(&sack.cpt2);

        if result.0 != 0 {
            // result = 2^n where n == bit index of the item in both compartments
            // -> priority = n+1 (index 0 == letter 'a' -> 1)
            sum += result.first_true_bit_index() + 1;
        }
    }

    Ok(sum.to_string())
}

#[derive(Default)]
struct ThreeSacks {
    pub sack1: LetterField,
    pub sack2: LetterField,
    pub sack3: LetterField,
}

impl From<&[&str; 3]> for ThreeSacks {
    fn from(content: &[&str; 3]) -> Self {
        Self {
            sack1: LetterField::from(content[0].chars()),
            sack2: LetterField::from(content[1].chars()),
            sack3: LetterField::from(content[2].chars()),
        }
    }
}

pub fn run2(input: &str) -> anyhow::Result<String> {
    let mut sum: usize = 0;

    for chunk in input.lines().array_chunks::<3>() {
        let three_sacks = ThreeSacks::from(&chunk);
        let result = three_sacks
            .sack1
            .combine_with(&three_sacks.sack2.combine_with(&three_sacks.sack3));

        assert_ne!(result.0, 0);

        sum += result.first_true_bit_index() + 1;
    }

    Ok(sum.to_string())
}
