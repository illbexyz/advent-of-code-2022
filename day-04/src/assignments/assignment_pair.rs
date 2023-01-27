use std::str::FromStr;

use super::assignment::Assignment;

#[derive(Debug)]
pub struct AssignmentPair {
    first: Assignment,
    second: Assignment,
}

impl AssignmentPair {
    pub fn overlaps(&self) -> bool {
        Self::overlaps_condition(&self.first, &self.second)
            || Self::overlaps_condition(&self.second, &self.first)
    }

    fn overlaps_condition(first: &Assignment, second: &Assignment) -> bool {
        (second.from >= first.from && second.from <= first.to)
            || (second.to <= first.to && second.to >= first.from)
    }

    pub fn fully_overlaps(&self) -> bool {
        Self::fully_overlaps_condition(&self.first, &self.second)
            || Self::fully_overlaps_condition(&self.second, &self.first)
    }

    fn fully_overlaps_condition(first: &Assignment, second: &Assignment) -> bool {
        second.from >= first.from && second.to <= first.to
    }
}

impl FromStr for AssignmentPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<_> = s.split(",").take(2).collect();

        let (first_str, second_str) = match values[..] {
            [f, s] => (f, s),
            _ => return Err("".to_string()),
        };

        let first: Assignment = first_str.parse::<Assignment>().map_err(|e| e.to_string())?;
        let second: Assignment = second_str
            .parse::<Assignment>()
            .map_err(|e| e.to_string())?;

        Ok(Self { first, second })
    }
}
