use std::collections::HashSet;

fn find_non_repeating_of_len(line: &str, n: usize) -> Option<usize> {
    for i in n..line.len() {
        let slice = &line[i - n..i];
        let set: HashSet<char> = slice.chars().collect();

        if set.len() == n {
            return Some(i);
        }
    }

    None
}

pub struct Packet {
    line: String,
}

impl Packet {
    pub fn new(line: &str) -> Self {
        Self {
            line: line.to_string(),
        }
    }

    pub fn start_of_packet(&self) -> Option<usize> {
        find_non_repeating_of_len(&self.line, 4)
    }

    pub fn start_of_message(&self) -> Option<usize> {
        find_non_repeating_of_len(&self.line, 14)
    }
}
