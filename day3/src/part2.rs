pub fn part2(input: String) {
    let values: Vec<&str> = input.lines().collect();

    let oxy = solve(values.clone(), true);
    let co2 = solve(values.clone(), false);

    println!("oxy: {}, co2: {}", oxy, co2);
    println!(
        "multiple: {}",
        u32::from_str_radix(&oxy, 2).unwrap() * u32::from_str_radix(&co2, 2).unwrap()
    )
}

fn solve(mut values: Vec<&str>, is_oxy: bool) -> String {
    let n_bits = values[0].len();
    for i in 0..n_bits {
        let n = values.len();
        if n == 1 {
            break;
        };

        let sum = values.iter().fold(0, |s, str| {
            s + str.chars().nth(i).unwrap().to_digit(2).unwrap()
        });
        let most_common = if sum * 2 == n as u32 {
            1
        } else {
            (sum as f32 / n as f32).round() as u32
        };

        values.drain_filter(|bit_str| {
            let bit = bit_str.chars().nth(i).unwrap().to_digit(2);
            (bit == Some(most_common)) != is_oxy
        });
    }

    values[0].to_string()
}
