use std::collections::HashSet;

pub fn run(input: String) -> usize {
    for (i, window) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
        let mut hs = HashSet::new();
        let mut unique = true;
        for &c in window {
            match hs.insert(c) {
                false => {
                    unique = false;
                    break;
                }
                true => continue,
            }
        }
        if unique {
            return i + window.len();
        }
    }
    unreachable!()
}

#[test]
fn example_1() {
    assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5)
}

#[test]
fn example_2() {
    assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6)
}

#[test]
fn example_3() {
    assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10)
}

#[test]
fn example_4() {
    assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11)
}
