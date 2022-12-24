use tuple_map::*;

pub fn run(input: String) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split_once(',').unwrap().map(|ranges| {
                ranges
                    .split_once('-')
                    .unwrap()
                    .map(|n| n.parse::<u32>().unwrap())
            })
        })
        .map(|((a0, a1), (b0, b1))| {
            if a0 <= b0 && a1 >= b1 {
                b1 - b0
            } else if b0 <= a0 && b1 >= a1 {
                a1 - a0
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn example() {
    assert_eq!(
        run(r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#
        .trim()
        .to_string()),
        4
    );
}
