use std::collections::BTreeMap;
use std::fs::read_to_string;

pub fn solve() -> i32 {
    let input = read_to_string("day3/input.txt").expect("should read input");

    let priorities_map: BTreeMap<char, i32> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, (i as i32) + 1))
        .collect();

    let priorities_sum: i32 = input
        .lines()
        .map(|line| {
            let comp_size = line.len() / 2;
            let (comp_one, comp_two) = line.split_at(comp_size);
            let duplicate = comp_one.chars().find(|&c| comp_two.contains(c)).unwrap();
            let priority = priorities_map.get(&duplicate).unwrap();

            priority
        })
        .sum();

    priorities_sum
}
