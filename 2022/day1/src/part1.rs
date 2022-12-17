use std::cmp::max;

pub fn run(input: String) {
    let out = input
        .lines()
        .fold::<(i32, i32), _>((0, 0), |acc, e| match e {
            "" => (max(acc.0, acc.1), 0),
            v => (
                acc.0,
                acc.1 + v.parse::<i32>().expect("could not parse line into integer"),
            ),
        })
        .0;
    println!("{}", out)
}
