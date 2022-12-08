use std::fmt::Display;

pub struct Position {
    pub horizontal: i64,
    pub depth: i64,
    pub should_aim: bool,
    pub aim: i64,
}

impl Position {
    pub fn origin(should_aim: bool) -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
            should_aim,
        }
    }

    pub fn forward(&mut self, units: i64) {
        self.horizontal += units;
        if self.should_aim {
            self.depth += self.aim * units;
        }
    }

    pub fn up(&mut self, units: i64) {
        if self.should_aim {
            self.aim -= units;
        } else {
            self.depth -= units;
        }
    }

    pub fn down(&mut self, units: i64) {
        if self.should_aim {
            self.aim += units;
        } else {
            self.depth += units;
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.should_aim {
            write!(
                f,
                "({}, {}), aim: {}",
                self.horizontal, self.depth, self.aim
            )
        } else {
            write!(f, "({}, {})", self.horizontal, self.depth)
        }
    }
}
