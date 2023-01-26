use std::{collections::HashSet, fs::read_to_string};

fn priority_of_char(char: &char) -> usize {
    let digit = *char as usize;
    if char.is_uppercase() {
        digit - 38
    } else {
        digit - 96
    }
}

fn build_set(str: &str) -> HashSet<char> {
    str.chars().into_iter().collect()
}

#[derive(Clone)]
pub struct Rucksack {
    line: String,
}

impl Rucksack {
    pub fn from_str(str: &str) -> Self {
        Self {
            line: str.to_string(),
        }
    }

    pub fn priority(&self) -> usize {
        let (s1, s2) = self.line.split_at(self.line.len() / 2);

        let left = build_set(&s1);
        let right = build_set(&s2);

        left.intersection(&right)
            .map(|char| priority_of_char(&char))
            .sum()
    }
}

pub struct ElvesGroup {
    rucksacks: Vec<Rucksack>,
}

impl ElvesGroup {
    pub fn from_rucksacks(rucksacks: &[Rucksack]) -> Self {
        ElvesGroup {
            rucksacks: rucksacks.to_vec(),
        }
    }

    pub fn priority(&self) -> usize {
        let mut iter = self
            .rucksacks
            .iter()
            .map(|rucksack| build_set(&rucksack.line));

        let intersection = iter
            .next()
            .map(|set| iter.fold(set, |ref set1, ref set2| set1 & set2))
            .expect("At least one set expected");

        intersection.iter().map(|char| priority_of_char(char)).sum()
    }
}

fn main() -> Result<(), String> {
    let filename = "input.txt";
    let file_contents = read_to_string(filename).map_err(|err| err.to_string())?;

    let rucksacks: Vec<_> = file_contents
        .lines()
        .map(|line| Rucksack::from_str(&line))
        .collect();

    let part_1: usize = rucksacks.iter().map(|rucksack| rucksack.priority()).sum();

    println!("Part 1: {}", part_1);

    let elves_groups: Vec<_> = rucksacks
        .chunks_exact(3)
        .map(|rucksack_group| ElvesGroup::from_rucksacks(&rucksack_group))
        .collect();

    let part_2: usize = elves_groups.iter().map(|group| group.priority()).sum();

    println!("Part 2: {}", part_2);

    Ok(())
}
