pub fn part1(input: String) {
    let mut boards: Vec<[u32; 25]> = vec![];

    let mut iter = input.lines();
    let draws: Vec<u32> = iter
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.parse::<u32>().unwrap())
        .collect();

    loop {
        iter.next();
    }
    println!("{:?}", x);
    println!("{:?}", y);
}

fn parse_board(board_str: String) {}
