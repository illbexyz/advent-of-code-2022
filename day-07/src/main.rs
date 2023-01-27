use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Directory {
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
enum Cmd {
    Cd(String),
    Ls,
}

impl FromStr for Cmd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once("$ ").ok_or(format!(
            "Expected command to begin with $. Instead I got: {}",
            s
        ))?;

        // Two tokens
        if let Some((cmd, param)) = rest.split_once(" ") {
            return match (cmd, param) {
                ("cd", dirname) => Ok(Self::Cd(dirname.to_owned())),
                (_, _) => Err(format!("Unknown command: {}", rest)),
            };
        // One token
        } else if rest == "ls" {
            return Ok(Self::Ls);
        }

        Err(format!("Unknown command: {}", rest))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CmdResult {
    File(File),
    Directory(Directory),
}

impl FromStr for CmdResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (token_1, token_2) = s
            .split_once(" ")
            .ok_or(format!("Unexpected CmdResult: {}", s))?;

        match token_1 {
            "dir" => Ok(Self::Directory(Directory {
                name: token_2.to_owned(),
            })),
            num => {
                let file_size = num.parse::<u32>().map_err(|_| "")?;
                Ok(Self::File(File {
                    name: token_2.to_owned(),
                    size: file_size,
                }))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Cmd(Cmd),
    CmdResult(CmdResult),
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fst_char = s.chars().next().ok_or("")?;

        if fst_char == '$' {
            let cmd = s.parse::<Cmd>()?;
            Ok(Self::Cmd(cmd))
        } else {
            let cmd = s.parse::<CmdResult>()?;
            Ok(Self::CmdResult(cmd))
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    files: HashMap<String, u32>,
}

impl FileSystem {
    pub fn directories_smaller_than(&self, size: u32) -> Vec<(&String, &u32)> {
        self.files.iter().filter(|(_, &v)| v <= size).collect()
    }

    pub fn directory_to_delete(&self, space_needed: u32) -> Option<u32> {
        let unused_space = self.unused_space();
        let space_to_free = space_needed - unused_space;

        let &min_opt = self
            .files
            .values()
            .filter(|&&size| size >= space_to_free)
            .min()?;

        Some(min_opt)
    }

    pub fn unused_space(&self) -> u32 {
        70000000 - self.files.get("/").unwrap_or(&0)
    }
}

impl From<Vec<Line>> for FileSystem {
    fn from(lines: Vec<Line>) -> Self {
        let mut files_map: HashMap<String, u32> = HashMap::new();
        let mut curr_path = vec![String::from("/")];

        for line in lines {
            match line {
                Line::Cmd(Cmd::Ls) => (),
                Line::Cmd(Cmd::Cd(path)) if path == "/" => {
                    curr_path = vec![String::from("/")];
                }
                Line::Cmd(Cmd::Cd(path)) if path == ".." => {
                    curr_path.pop();
                }
                Line::Cmd(Cmd::Cd(path)) => {
                    curr_path.push(path);
                }
                Line::CmdResult(CmdResult::File(file)) => {
                    let mut path = String::new();

                    for dir in &curr_path {
                        path += &dir;

                        let curr_size = files_map.get(&path).unwrap_or(&0);
                        files_map.insert(path.clone(), curr_size + file.size);
                    }
                }
                Line::CmdResult(CmdResult::Directory(_)) => (),
            }
        }

        FileSystem { files: files_map }
    }
}

fn main() -> Result<(), String> {
    let filename = "input.txt";
    let file_content = std::fs::read_to_string(&filename).map_err(|e| e.to_string())?;

    let lines: Vec<Line> = file_content
        .lines()
        .map(|line| line.parse::<Line>())
        .collect::<Result<_, _>>()?;
    let file_system: FileSystem = lines.into();

    let part_1: u32 = file_system
        .directories_smaller_than(100000)
        .iter()
        .map(|(_, v)| **v)
        .sum();

    println!("Part 1: {}", part_1);

    let part_2: u32 = file_system
        .directory_to_delete(30000000)
        .ok_or("At least one file expected")?;

    println!("Part 2: {}", part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cd() {
        let cmd: Result<Cmd, _> = "$ cd /".parse();
        assert_eq!(cmd, Ok(Cmd::Cd("/".to_owned())))
    }

    #[test]
    fn parse_ls() {
        let cmd: Result<Cmd, _> = "$ ls".parse();
        assert_eq!(cmd, Ok(Cmd::Ls))
    }

    #[test]
    fn parse_file() {
        let cmd: Result<CmdResult, _> = "14848514 b.txt".parse();
        assert_eq!(
            cmd,
            Ok(CmdResult::File(File {
                name: String::from("b.txt"),
                size: 14848514
            }))
        )
    }

    #[test]
    fn parse_dir() {
        let cmd: Result<CmdResult, _> = "dir d".parse();
        assert_eq!(
            cmd,
            Ok(CmdResult::Directory(Directory {
                name: String::from("d")
            }))
        )
    }
}
