#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn from_letter(letter: &char) -> Option<Self> {
        match letter {
            'A' | 'X' => Some(Self::Rock),
            'B' | 'Y' => Some(Self::Paper),
            'C' | 'Z' => Some(Self::Scissors),
            _ => None,
        }
    }

    pub fn wins_vs(&self) -> Self {
        match &self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn loses_vs(&self) -> Self {
        match &self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}
