use std::collections::HashSet;

pub fn run(input: String) -> u32 {
    input
        .lines()
        .map::<char, _>(|l| {
            let n = l.len();
            let first = l.chars().take(n / 2);
            let second = l.chars().skip(n / 2);
            return *HashSet::<char>::from_iter(first)
                .intersection(&HashSet::from_iter(second))
                .next()
                .expect("no intersection found");
        })
        .map(|common| match common {
            'a'..='z' => 1 + common as u32 - 97,
            'A'..='Z' => 27 + common as u32 - 65,
            _ => unreachable!(),
        })
        .sum::<u32>()
}

#[test]
fn example() {
    let input = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
    .trim();
    assert_eq!(run(input.to_string()), 157);
}
