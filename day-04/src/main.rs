use std::fs::read_to_string;

mod assignments;
use crate::assignments::AssignmentList;

fn main() -> Result<(), String> {
    let filename = "input.txt";
    let file_content: String = read_to_string(&filename).map_err(|err| err.to_string())?;
    let assignments_list: AssignmentList = file_content.parse()?;

    let part_1 = assignments_list.full_overlaps_count();

    println!("Part 1: {}", part_1);

    let part_2 = assignments_list.overlaps_count();

    println!("Part 2: {}", part_2);

    Ok(())
}
