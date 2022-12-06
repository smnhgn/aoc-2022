use std::fs::read_to_string;

pub fn solve() -> i32 {
    let input = read_to_string("day5/input.txt").expect("should read input");

    let separator_pos = input.lines().position(|line| line.is_empty()).unwrap();

    let ship_lines = input.lines().take(separator_pos).collect::<Vec<_>>();
    let ship_width = ship_lines
        .last()
        .unwrap()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<_>>()
        .len();
    let max_crates = ship_lines
        .into_iter()
        .map(|line| {
            line.chars()
                .filter(|char| char.is_alphabetic())
                .collect::<Vec<_>>()
                .len()
        })
        .sum();

    let ship: Vec<Vec<char>> = vec![vec![' '; max_crates]; ship_width];

    // let instruction_lines = input.lines().skip(separator_pos).collect::<Vec<_>>();

    // let ship = ship_lines
    //     .into_iter()
    //     .map(|line| {
    //         line.chars()
    //             .into_iter()
    //             .enumerate()
    //             .filter(|(pos, _)| *pos != 0 && (*pos - 1) % 4 == 0)
    //             .map(|(_, char)| char)
    //             .collect::<Vec<_>>()
    //     })
    //     .rev()
    //     .map(|line| line)
    //     .collect::<Vec<_>>();

    println!("{:?} {:?} {:?}", ship_width, max_crates, ship);
    1
}
