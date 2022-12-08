use std::ops::Div;

pub fn part1(input: String) {
    let mut n_lines = 0;
    let n_bits = input.find('\n').unwrap();
    let mut bit_avgs: Vec<u32> = vec![0; n_bits];

    for line in input.lines() {
        n_lines += 1;
        for (i, b) in line.chars().enumerate() {
            let bit = b.to_digit(2).expect("Expected a binary number.");
            bit_avgs[i] += bit;
        }
    }

    let mut epsilon_bin_str = String::from("");
    let gamma = isize::from_str_radix(
        &bit_avgs
            .into_iter()
            .fold::<String, _>("".to_string(), |mut bin, n| {
                let bit = (n as f32).div(n_lines as f32).round() as u32;
                epsilon_bin_str.push_str(&(1 - bit).to_string());
                bin.push_str(&bit.to_string());
                bin
            }),
        2,
    )
    .unwrap();
    let epsilon = isize::from_str_radix(&epsilon_bin_str, 2).unwrap();

    println!("gamma: {}, epsilon: {}", gamma, epsilon);
    println!("power consumption: {}", gamma * epsilon);
}
