#[derive(Debug)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn from_letter(letter: &char) -> Option<Self> {
        match letter {
            'X' => Some(Self::Lose),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }
}
