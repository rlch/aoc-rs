use std::collections::HashSet;

pub fn run(input: String) -> usize {
    for (i, window) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
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
