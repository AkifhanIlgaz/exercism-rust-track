// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn move_to(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                y: self.y + 1,
                ..self
            },
            Direction::East => Self {
                x: self.x + 1,
                ..self
            },
            Direction::South => Self {
                y: self.y - 1,
                ..self
            },
            Direction::West => Self {
                x: self.x - 1,
                ..self
            },
        }
    }
}

pub struct Robot {
    location: Point,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            location: Point::new(x, y),
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_direction = match &self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Self {
            direction: new_direction,
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_direction = match &self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };

        Self {
            direction: new_direction,
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            location: self.location.move_to(self.direction),
            direction: self.direction,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut new = Self { ..self };
        for instruction in instructions.chars() {
            match instruction {
                'L' => new = new.turn_left(),
                'R' => new = new.turn_right(),
                'A' => new = new.advance(),
                _ => unreachable!(),
            }
        }
        new
    }

    pub fn position(&self) -> (i32, i32) {
        (self.location.x, self.location.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
