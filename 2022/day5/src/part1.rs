pub struct State {
    pub instructions: Vec<Instruction>,
    pub crates: Vec<String>,
}

pub struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let count = parts.nth(1).unwrap();
            let from = parts.nth(1).unwrap();
            let to = parts.nth(1).unwrap();
            Instruction {
                count: count.parse().unwrap(),
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
            }
        })
        .collect()
}

fn parse_crates(input: String) -> Vec<String> {
    let n = input.len() / 4;
    let mut out = vec![String::new(); n];
    input.lines().for_each(|l| {
        let parts = l.chars().skip(1).step_by(4);
        for (i, p) in parts.enumerate() {
            match p {
                'A'..='Z' => out[i].insert(0, p),
                '0'..='9' | ' ' => continue,
                _ => unreachable!(),
            }
        }
    });
    out
}

pub fn parse(input: String) -> State {
    let crates_input = input
        .lines()
        .clone()
        .take_while(|l| !l.is_empty())
        .fold(String::new(), |s, l| s + l + "\n")
        .trim_end()
        .to_string();
    let instructions_input = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .fold(String::new(), |s, l| s + l + "\n")
        .trim_end()
        .to_string();
    State {
        instructions: parse_instructions(instructions_input),
        crates: parse_crates(crates_input),
    }
}

pub fn run(input: String) -> String {
    let mut state = parse(input);
    for instruction in state.instructions {
        for _ in 0..instruction.count {
            let next = state.crates[instruction.from - 1].pop().unwrap();
            state.crates[instruction.to - 1].push(next);
        }
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
    assert_eq!(run(input.to_string()), "CMZ".to_string());
}
