use core::str::FromStr;

use super::assignment_pair::AssignmentPair;

#[derive(Debug)]
pub struct AssignmentList {
    assignments: Vec<AssignmentPair>,
}

impl AssignmentList {
    pub fn new(assignments: Vec<AssignmentPair>) -> Self {
        Self { assignments }
    }

    pub fn overlaps_count(&self) -> usize {
        self.assignments
            .iter()
            .filter(|pair| pair.overlaps())
            .count()
    }

    pub fn full_overlaps_count(&self) -> usize {
        self.assignments
            .iter()
            .filter(|pair| pair.fully_overlaps())
            .count()
    }
}

impl FromStr for AssignmentList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let assignments: Vec<_> = s
            .lines()
            .map(|line| line.parse::<AssignmentPair>())
            .collect::<Result<_, _>>()?;

        Ok(AssignmentList::new(assignments))
    }
}
