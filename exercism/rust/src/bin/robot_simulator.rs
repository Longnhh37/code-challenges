#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            position: (x, y),
            direction,
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.direction = self.direction.turn_right();
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.direction = self.direction.turn_left();
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        let (x, y) = match self.direction {
            Direction::North => (self.position.0, self.position.1 + 1),
            Direction::East => (self.position.0 + 1, self.position.1),
            Direction::South => (self.position.0, self.position.1 - 1),
            Direction::West => (self.position.0 - 1, self.position.1),
        };

        self.position = (x, y);
        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.bytes().fold(self, |acc, b| match b {
            b'R' => acc.turn_right(),
            b'L' => acc.turn_left(),
            b'A' => acc.advance(),
            _ => unreachable!(),
        })
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }
}
fn main() {}
