use std::fs::read_to_string;

pub fn solve() -> String {
    let input = read_to_string("day5/input.txt").expect("should read input");

    let separator_pos = input.lines().position(|line| line.is_empty()).unwrap();

    let mut ship_crates: Vec<Vec<char>> = vec![];
    let crate_lines = input.lines().take(separator_pos - 1).collect::<Vec<_>>();
    let instruction_lines = input.lines().skip(separator_pos + 1).collect::<Vec<_>>();

    for (crate_line_index, crate_line) in crate_lines.into_iter().rev().enumerate() {
        let crate_chars = crate_line
            .chars()
            .enumerate()
            .filter(|(pos, _)| *pos != 0 && (*pos - 1) % 4 == 0)
            .map(|(_, char)| char)
            .collect::<Vec<_>>();

        for (crate_char_index, crate_char) in crate_chars.into_iter().enumerate() {
            if crate_line_index == 0 {
                ship_crates.push(vec![]);
            }

            if crate_char.is_alphabetic() {
                let ship_crate = &mut ship_crates[crate_char_index];

                ship_crate.push(crate_char);
            }
        }
    }

    for instruction_line in instruction_lines {
        let instructions = instruction_line
            .split_whitespace()
            .into_iter()
            .filter_map(|part| part.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let amount = instructions[0];
        let from = instructions[1] - 1;
        let to = instructions[2] - 1;

        for _ in 1..=amount {
            let crate_item = ship_crates[from].pop().unwrap();

            ship_crates[to].push(crate_item);
        }
    }

    let ship_crates_top = ship_crates
        .into_iter()
        .map(|ship_crate| ship_crate.last().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("");

    ship_crates_top
}
