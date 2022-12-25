use crate::part1::parse;

pub fn run(input: String) -> String {
    let mut state = parse(input);
    for instruction in state.instructions {
        let c = state.crates.get_mut(instruction.from - 1).unwrap();
        let n = c.len();
        let removed = c.drain((n - instruction.count)..n).collect::<String>();
        state.crates[instruction.to - 1].push_str(&removed);
    }
    state
        .crates
        .iter()
        .filter_map(|c| c.chars().last().map(|c| c.to_string()))
        .collect()
}

#[test]
fn example() {
    let input = r#"    [D]
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#
    .trim_end();
    assert_eq!(run(input.to_string()), "MCD".to_string());
}
