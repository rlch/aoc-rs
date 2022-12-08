use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    sequence::separated_pair,
    IResult,
};

pub struct Vent {
    pub from: (u32, u32),
    pub to: (u32, u32),
}

fn parse_coord(input: &str) -> IResult<&str, (u32, u32), ()> {
    match separated_pair::<&str, &str, char, &str, (), _, _, _>(digit1, char(','), digit1)(input) {
        Ok((trailing, (from, to))) => Ok((
            trailing,
            (from.parse::<u32>().unwrap(), to.parse::<u32>().unwrap()),
        )),
        Err(_) => Err(nom::Err::Error(())),
    }
}

impl Vent {
    pub fn parse(input_line: &str) -> Self {
        match separated_pair(parse_coord, tag(" -> "), parse_coord)(input_line) {
            Ok((_, (from, to))) => Self { from, to },
            Err(_) => panic!("Could not parse input"),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.from.1 == self.to.1
    }

    pub fn is_vertical(&self) -> bool {
        self.from.0 == self.to.0
    }

    pub fn is_line(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn is_diagonal(&self) -> bool {
        self.to.1.abs_diff(self.from.1) == self.to.0.abs_diff(self.from.0)
    }

    pub fn spanning_set(&self, diagonal: bool) -> Vec<(u32, u32)> {
        let mut set: Vec<(u32, u32)> = vec![];

        let x0 = self.from.0.min(self.to.0);
        let x1 = self.from.0.max(self.to.0);
        let y0 = self.from.1.min(self.to.1);
        let y1 = self.from.1.max(self.to.1);

        if self.is_vertical() {
            assert_eq!(x0, x1);
            for j in y0..=y1 {
                set.push((x0, j));
            }
        } else if self.is_horizontal() {
            assert_eq!(y0, y1);
            for i in x0..=x1 {
                set.push((i, y0));
            }
        } else if diagonal && self.is_diagonal() {
            for i in 0..=(x1 - x0) {
                let mut y_dir = (self.to.1 as i64 - self.from.1 as i64).signum();
                let y_start = if self.to.0 > self.from.0 {
                    self.from.1
                } else {
                    y_dir *= -1;
                    self.to.1
                } as i64;

                set.push((x0 + i, (y_start + y_dir * i as i64) as u32))
            }
        } else {
            for i in x0..=x1 {
                for j in y0..=y1 {
                    set.push((i, j));
                }
            }
        }

        set
    }
}

impl Display for Vent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.from, self.to)
    }
}
