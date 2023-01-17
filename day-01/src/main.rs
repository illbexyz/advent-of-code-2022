use std::fs::read_to_string;
use std::io::Error;

struct Elves {
    weights: Vec<Vec<u32>>,
}

impl Elves {
    fn new(weights: Vec<Vec<u32>>) -> Self {
        Self { weights }
    }
}

fn weights_by_elf(contents: &str) -> Elves {
    let elves_weights = contents.split("\n\n");

    let weights = elves_weights
        .map(|elf_weights| {
            elf_weights
                .split_whitespace()
                .map(|weight| weight.parse::<u32>().expect("Unexpected character"))
                .collect()
        })
        .collect();

    Elves::new(weights)
}

fn part_1(elves: &Elves) -> Option<u32> {
    elves.weights.iter().map(|w| w.iter().sum()).max()
}

fn part_2(elves: &Elves) -> Option<u32> {
    let mut x = elves
        .weights
        .iter()
        .map(|w| w.iter().sum())
        .collect::<Vec<u32>>();

    x.sort_by(|x, y| y.cmp(x));

    Some(x[..3].iter().sum::<u32>())
}

fn main() -> Result<(), Error> {
    let filepath = "input.txt";
    let contents = read_to_string(filepath).expect("The file doesn't exists");

    let elves = weights_by_elf(&contents);
    let part_1_result = part_1(&elves).expect("Something went wrong with part 1");

    println!("Part 1: {}", part_1_result);

    let part_2_result = part_2(&elves).expect("Something went wrong with part 2");

    println!("Part 2: {}", part_2_result);

    Ok(())
}
