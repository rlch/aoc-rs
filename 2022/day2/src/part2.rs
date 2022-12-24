use crate::part1::Shape;

fn get_conjugate_shape(opponent: Shape, player: Shape) -> Shape {
    match player {
        // Lose
        Shape::Rock => match opponent {
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
            Shape::Rock => Shape::Scissors,
        },
        // Draw
        Shape::Paper => opponent,
        // Win
        Shape::Scissors => match opponent {
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
            Shape::Rock => Shape::Paper,
        },
    }
}

pub fn run(input: String) -> impl std::fmt::Display {
    input
        .lines()
        .map::<(Shape, Shape), _>(|l| {
            let shapes = l.split_once(' ').expect("invalid format");
            (
                shapes.0.into(),
                get_conjugate_shape(shapes.0.into(), shapes.1.into()),
            )
        })
        .fold(0, |acc, shapes| acc + shapes.1.play(shapes.0))
}
