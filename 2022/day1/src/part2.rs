use log::debug;

pub fn run(input: String) {
    let out: i32 = input
        .lines()
        .fold::<([i32; 3], i32), _>(([0, 0, 0], 0), |acc, e| match e {
            "" => {
                let mut sorted = [&acc.0[..], &[acc.1]].concat();
                sorted.sort_by(|a, b| b.cmp(a));
                debug!("sorted: {:?}", sorted);
                ([sorted[0], sorted[1], sorted[2]], 0)
            }
            v => (
                acc.0,
                acc.1 + v.parse::<i32>().expect("could not parse line into integer"),
            ),
        })
        .0
        .iter()
        .sum();
    println!("{:?}", out)
}
