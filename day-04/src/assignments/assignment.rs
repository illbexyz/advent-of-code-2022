use core::str::FromStr;

#[derive(Debug)]
pub struct Assignment {
    pub from: u8,
    pub to: u8,
}

impl Assignment {
    pub fn new(from: u8, to: u8) -> Self {
        Self { from, to }
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<_> = s.split("-").take(2).collect();

        let (from_str, to_str) = match values[..] {
            [x, y] => (x, y),
            _ => return Err("Expected two values x-y".to_string()),
        };

        let from: u8 = from_str.parse::<u8>().map_err(|e| e.to_string())?;
        let to: u8 = to_str.parse::<u8>().map_err(|e| e.to_string())?;

        Ok(Self::new(from, to))
    }
}
