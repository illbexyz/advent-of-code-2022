mod game;
use std::fs::read_to_string;

use crate::game::{Game, Part};

fn main() -> Result<(), String> {
    let filename = "input.txt";
    let file_content = read_to_string(filename).map_err(|err| err.to_string())?;

    let score_p1 = Game::from_file_content(&file_content, Part::One)?.score();
    let score_p2 = Game::from_file_content(&file_content, Part::Two)?.score();

    println!("Part 1: {}", score_p1);
    println!("Part 2: {}", score_p2);

    Ok(())
}
