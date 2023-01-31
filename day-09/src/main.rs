use std::fmt::Display;
use std::{collections::HashSet, fs::read_to_string, str::FromStr};

mod vec2;

use crate::vec2::Vec2;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Right => "R",
            Direction::Left => "L",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Unexpected character".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub direction: Direction,
    pub length: u32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, len_str) = s
            .split_once(" ")
            .ok_or_else(|| "Expected 2 characters separated by ' '".to_string())?;

        let direction = dir_str.parse::<Direction>()?;
        let length = len_str.parse::<u32>().map_err(|err| err.to_string())?;

        Ok(Self { direction, length })
    }
}

impl From<&Direction> for Vec2 {
    fn from(instruction: &Direction) -> Self {
        match instruction {
            Direction::Up => Vec2::new(1, 0),
            Direction::Down => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(0, 1),
            Direction::Left => Vec2::new(0, -1),
        }
    }
}

pub struct RopeBridge {}

impl RopeBridge {
    pub fn simulate_knots(instructions: &[Instruction], knots_count: usize) -> u32 {
        let mut knots = vec![Vec2::new(0, 0); knots_count];
        let mut tail_positions: HashSet<Vec2> = HashSet::from([knots[0]]);

        // println!("== Initial state ==\n");
        // Self::print_state(&knots);

        for instruction in instructions {
            let vec_dir: Vec2 = (&instruction.direction).into();

            // println!("== {} {} ==\n", instruction.direction, instruction.length);

            for _ in 0..instruction.length {
                knots[0] = knots[0] + vec_dir;

                for idx in 1..knots_count {
                    if !knots[idx - 1].is_near(knots[idx]) {
                        let sum = Self::get_movement_forward(knots[idx - 1], knots[idx]);

                        knots[idx] = knots[idx - 1] + sum;

                        if idx == knots_count - 1 {
                            tail_positions.insert(knots[idx]);
                        }
                    }
                }

                // Self::print_state(&knots)
            }
        }

        tail_positions.len() as u32
    }

    #[allow(dead_code)]
    fn print_state(knots: &[Vec2]) {
        for i in (0..5).rev() {
            for j in 0..6 {
                let v = knots.iter().position(|p| p.x == i && p.y == j);

                let symbol = match v {
                    Some(idx) => idx.to_string(),
                    None => ".".to_string(),
                };

                print!("{}", symbol);
            }
            println!()
        }
        println!()
    }

    fn get_movement_forward(from: Vec2, to: Vec2) -> Vec2 {
        let sub = to - from;

        let to_x = if from.x < to.x {
            Vec2::new(1, 0)
        } else {
            Vec2::new(-1, 0)
        };

        let to_y = if from.y < to.y {
            Vec2::new(0, 1)
        } else {
            Vec2::new(0, -1)
        };

        if sub.x.abs() > 1 && sub.y.abs() > 1 {
            to_x + to_y
        } else if sub.x.abs() > 1 {
            to_x
        } else {
            to_y
        }
    }
}

fn main() -> Result<(), String> {
    let filename = "input.txt";
    let file_content = read_to_string(filename).map_err(|e| e.to_string())?;

    let instructions: Vec<Instruction> = file_content
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<_, _>>()?;

    let part_1 = RopeBridge::simulate_knots(&instructions, 2);

    println!("Part 1: {}", part_1);

    let part_2 = RopeBridge::simulate_knots(&instructions, 10);

    println!("Part 2: {}", part_2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_near() {
        assert_eq!(vec2!(1, 4).is_near(vec2!(0, 3)), true);
    }
}
