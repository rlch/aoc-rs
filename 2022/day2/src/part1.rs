pub enum Shape {
    Rock,     // A/X - 1
    Paper,    // B/Y - 2
    Scissors, // C/Z - 3
}

pub const WIN: u32 = 6;
pub const DRAW: u32 = 3;
pub const LOSS: u32 = 0;

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    pub fn play(&self, against: Shape) -> u32 {
        self.score()
            + match (self, &against) {
                (Shape::Rock, Shape::Scissors) => WIN,
                (Shape::Paper, Shape::Rock) => WIN,
                (Shape::Scissors, Shape::Paper) => WIN,
                (Shape::Rock, Shape::Rock) => DRAW,
                (Shape::Paper, Shape::Paper) => DRAW,
                (Shape::Scissors, Shape::Scissors) => DRAW,
                (_, _) => LOSS,
            }
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

pub fn run(input: String) -> u32 {
    input
        .lines()
        .map::<(Shape, Shape), _>(|l| {
            let shapes = l.split_once(' ').expect("invalid format");
            (shapes.0.into(), shapes.1.into())
        })
        .fold(0, |acc, shapes| acc + shapes.1.play(shapes.0))
}

#[test]
fn example() {
    assert_eq!(
        run(r#"
A Y
B X
C Z
"#
        .trim()
        .to_string()),
        15
    )
}

#[test]
fn draws() {
    assert_eq!(
        run(r#"
A X
B Y
C Z
A X
B Y
C Z
"#
        .trim()
        .to_string()),
        30
    )
}
