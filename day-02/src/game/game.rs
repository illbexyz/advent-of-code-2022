use super::turn::Turn;

pub enum Part {
    One,
    Two,
}

pub struct Game {
    turns: Vec<Turn>,
}

impl Game {
    pub fn score(&self) -> usize {
        self.turns.iter().map(|turn| turn.score()).sum()
    }

    pub fn from_file_content(file_content: &str, part: Part) -> Result<Self, String> {
        let turns: Vec<Turn> = file_content
            .lines()
            .map(|line| match part {
                Part::One => Turn::from_line_with_choice(line),
                Part::Two => Turn::from_line_with_outcome(line),
            })
            .collect::<Result<_, _>>()?;

        Ok(Game { turns })
    }
}
