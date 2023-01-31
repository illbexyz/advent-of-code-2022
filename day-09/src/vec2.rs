use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn sum(&self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    pub fn subtract(&self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }

    pub fn is_near(&self, v: Self) -> bool {
        let s = *self - v;

        return s.x.abs() <= 1 && s.y.abs() <= 1;
    }
}

#[macro_export]
macro_rules! vec2 {
    ($x: expr, $y: expr) => {
        Vec2::new($x, $y)
    };
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        self.sum(rhs)
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        self.subtract(rhs)
    }
}
