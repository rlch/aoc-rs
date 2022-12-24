use std::collections::HashSet;

pub fn run(input: String) -> u32 {
    let mut sum = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    for i in (0..input.lines().count()).step_by(3) {
        let a = HashSet::<char>::from_iter(lines[i].chars());
        let b = HashSet::<char>::from_iter(lines[i + 1].chars());
        let c = HashSet::<char>::from_iter(lines[i + 2].chars());
        let common = *HashSet::from_iter(a.intersection(&b).copied())
            .intersection(&c)
            .next()
            .expect("no intersection found");
        sum += match common {
            'a'..='z' => 1 + common as u32 - 97,
            'A'..='Z' => 27 + common as u32 - 65,
            _ => unreachable!(),
        }
    }
    sum
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
    assert_eq!(run(input.to_string()), 70);
}
