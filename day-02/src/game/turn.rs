use super::{choice::Choice, outcome::Outcome};

#[derive(Debug)]
pub struct Turn {
    opponent_choice: Choice,
    my_choice: Choice,
}

impl Turn {
    pub fn score(&self) -> usize {
        let turn_score = match (self.my_choice, self.opponent_choice) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => 6,
            (m, o) if m == o => 3,
            (_, _) => 0,
        };
        let choice_score = self.my_choice.score();

        choice_score + turn_score
    }

    pub fn from_line_with_choice(line: &str) -> Result<Self, String> {
        let (first_char, second_char) = Self::chars_from_line(line)?;

        let opponent_choice = Choice::from_letter(&first_char)
            .ok_or(format!("Unexpected character: '{}'", first_char))?;
        let my_choice = Choice::from_letter(&second_char)
            .ok_or(format!("Unexpected character: '{}'", second_char))?;

        Ok(Turn {
            opponent_choice,
            my_choice,
        })
    }

    pub fn from_line_with_outcome(line: &str) -> Result<Self, String> {
        let (first_char, second_char) = Self::chars_from_line(line)?;

        let opponent = Choice::from_letter(&first_char)
            .ok_or(format!("Unexpected character: '{}'", first_char))?;
        let instruction = Outcome::from_letter(&second_char)
            .ok_or(format!("Unexpected character: '{}'", second_char))?;

        let me = match &instruction {
            Outcome::Draw => opponent.clone(),
            Outcome::Lose => opponent.wins_vs(),
            Outcome::Win => opponent.loses_vs(),
        };

        Ok(Turn {
            opponent_choice: opponent,
            my_choice: me,
        })
    }

    fn chars_from_line(line: &str) -> Result<(char, char), String> {
        let first_char = line.chars().nth(0).ok_or("Unexpected missing line[0]")?;
        let second_char = line.chars().nth(2).ok_or("Unexpected missing line[2]")?;

        Ok((first_char, second_char))
    }
}
