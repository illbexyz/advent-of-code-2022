use itertools::iproduct;
use rayon::prelude::*;
use std::{fs::read_to_string, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeGrid {
    trees: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl TreeGrid {
    pub fn new(trees: Vec<u8>, rows: usize, cols: usize) -> Self {
        Self { trees, rows, cols }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.trees
            .get(x * self.cols as usize + y)
            .expect("Outside the grid")
            .to_owned()
    }

    pub fn get_visible_trees(&self) -> usize {
        // Start the count with the trees at the edges
        let trees_at_the_edges = (self.rows * 2 + self.cols * 2) - 4;

        let visible_trees_inside: usize = iproduct!(1..(self.rows - 1), 1..(self.cols - 1))
            .par_bridge()
            .map(|(x, y)| {
                let tree_height = self.get(x, y);

                let north: Vec<_> = self.trees_in_direction(x, y, Direction::North);
                let south: Vec<_> = self.trees_in_direction(x, y, Direction::South);
                let west: Vec<_> = self.trees_in_direction(x, y, Direction::West);
                let east: Vec<_> = self.trees_in_direction(x, y, Direction::East);

                let n = north.iter().filter(|&&h| h >= tree_height).count();
                let s = south.iter().filter(|&&h| h >= tree_height).count();
                let w = west.iter().filter(|&&h| h >= tree_height).count();
                let e = east.iter().filter(|&&h| h >= tree_height).count();

                if n == 0 || s == 0 || w == 0 || e == 0 {
                    1
                } else {
                    0
                }
            })
            .sum();

        trees_at_the_edges + visible_trees_inside
    }

    pub fn max_scenic_score(&self) -> usize {
        iproduct!(1..(self.rows - 1), 1..(self.cols - 1))
            .par_bridge()
            .map(|(x, y)| {
                let tree_height = self.get(x, y);

                let n = self.trees_in_sight(x, y, tree_height, Direction::North);
                let s = self.trees_in_sight(x, y, tree_height, Direction::South);
                let w = self.trees_in_sight(x, y, tree_height, Direction::West);
                let e = self.trees_in_sight(x, y, tree_height, Direction::East);

                let scenic_score = n * s * w * e;

                scenic_score
            })
            .max()
            .expect("At least one value is expected")
    }

    fn trees_in_direction(&self, x: usize, y: usize, dir: Direction) -> Vec<u8> {
        match dir {
            Direction::North => (0..x).map(|idx| self.get(idx, y)).rev().collect(),
            Direction::South => ((x + 1)..self.rows).map(|idx| self.get(idx, y)).collect(),
            Direction::West => (0..y).map(|idx| self.get(x, idx)).rev().collect(),
            Direction::East => ((y + 1)..self.cols).map(|idx| self.get(x, idx)).collect(),
        }
    }

    fn trees_in_sight(&self, x: usize, y: usize, curr_height: u8, dir: Direction) -> usize {
        let trees: Vec<_> = self.trees_in_direction(x, y, dir);
        let mut tree_count = trees.iter().take_while(|&&h| h < curr_height).count();

        let count_to_the_end = match dir {
            Direction::North => x,
            Direction::South => self.rows - x - 1,
            Direction::West => y,
            Direction::East => self.cols - y - 1,
        };

        let has_reached_the_end = tree_count == count_to_the_end;

        if !has_reached_the_end {
            tree_count += 1
        }

        tree_count
    }
}

impl FromStr for TreeGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let rows = lines.len();
        let cols = lines[0].len();
        let trees: Vec<u8> = lines
            .join("")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        Ok(TreeGrid::new(trees, rows, cols))
    }
}

fn main() -> Result<(), String> {
    let filename = "input.txt";
    let file_content = read_to_string(&filename).map_err(|e| e.to_string())?;
    let tree_grid: TreeGrid = file_content.parse()?;

    let part_1 = tree_grid.get_visible_trees();

    println!("Part 1: {part_1}");

    let part_2 = tree_grid.max_scenic_score();

    println!("Part 2: {part_2}");

    Ok(())
}
