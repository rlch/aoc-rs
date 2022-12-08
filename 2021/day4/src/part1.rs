use std::ops::Mul;

pub struct Outcome {
    pub unmarked_numbers: Vec<u32>,
    pub winning_number: u32,
}

pub fn part1(input: String) {
    let mut boards: Vec<[u32; 25]> = vec![];

    let mut iter = input.lines();
    let draws: Vec<u32> = iter
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.parse::<u32>().unwrap())
        .collect();

    let mut board_n = 0usize;
    let mut i = 0usize;
    for row in iter {
        // Implies the next 5 rows are another board.
        if row.is_empty() {
            boards.push([0u32; 25]);
            i = 0;
            board_n += 1;
        }
        // [i]'th row of the board
        else {
            for (j, col) in row
                .split_whitespace()
                .map(|d| d.parse::<u32>().unwrap())
                .enumerate()
            {
                boards[board_n - 1][5 * i + j] = col;
            }
            i += 1;
        }
    }

    let outcome: Option<Outcome> = {
        // Could probably use a bitmask. Eh
        let mut matches = vec![[false; 25]; boards.len()];
        let mut bingo = false;
        let mut winning_board_n: usize = 0;
        let mut winning_number: u32 = 0;

        for draw in &draws {
            for (n, board) in boards.iter().enumerate() {
                matches[n] = board.zip(matches[n]).map(|t| t.1 || t.0 == *draw);

                for i in 0..5 {
                    // Check horizontal matches from [i]th row.
                    if matches[n].iter().skip(i * 5).take(5).all(|e| *e) {
                        bingo = true;
                        break;
                    }

                    // Check vertical matches from [i]th col.
                    if matches[n].iter().skip(i).step_by(5).all(|e| *e) {
                        bingo = true;
                        break;
                    }
                }

                if bingo {
                    winning_board_n = n;
                    break;
                }
            }
            if bingo {
                winning_number = *draw;
                break;
            }
        }

        if bingo {
            let unmarked_numbers: Vec<u32> = matches[winning_board_n]
                .zip(boards[winning_board_n])
                .iter()
                .filter_map::<u32, _>(|tup| if tup.0 { None } else { Some(tup.1) })
                .collect();

            Some(Outcome {
                winning_number,
                unmarked_numbers,
            })
        } else {
            None
        }
    };

    match outcome {
        Some(o) => println!(
            "Winning number: {}\nUnmarked numbers: {:?}\nScore: {}",
            o.winning_number,
            o.unmarked_numbers,
            o.winning_number.mul(o.unmarked_numbers.iter().sum::<u32>())
        ),
        None => println!("No bingo :("),
    }
}
