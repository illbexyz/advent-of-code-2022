use std::error::Error;
use std::fs::read_to_string;

mod packet;

use crate::packet::Packet;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "input.txt";
    let file_content = read_to_string(filename)?;
    let line = file_content
        .lines()
        .next()
        .ok_or("Missing first line on input")?;

    let packet = Packet::new(&line);

    let part_1 = packet
        .start_of_packet()
        .ok_or("Can't find start of packet")?;

    println!("Part 1: {}", part_1);

    let part_2 = packet
        .start_of_message()
        .ok_or("Can't find start of message")?;

    println!("Part 2: {}", part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let packet = Packet::new("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(packet.start_of_packet(), Some(5))
    }

    #[test]
    fn two() {
        let packet = Packet::new("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(packet.start_of_packet(), Some(6))
    }

    #[test]
    fn three() {
        let packet = Packet::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(packet.start_of_packet(), Some(10))
    }

    #[test]
    fn four() {
        let packet = Packet::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(packet.start_of_packet(), Some(11))
    }
}
