use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn is_contained(sect: RangeInclusive<i32>, start: i32, end: i32) -> bool {
    sect.contains(&start) && sect.contains(&end)
}

pub fn solve() -> i32 {
    let input = read_to_string("./src/day4/_input.txt").expect("should read input");

    let sum = input
        .lines()
        .map(|line| {
            let sect_bounds: Vec<&str> = line.split_terminator(&[',', '-']).collect();

            let sect1_start = sect_bounds[0].parse::<i32>().unwrap();
            let sect1_end = sect_bounds[1].parse::<i32>().unwrap();
            let sect2_start = sect_bounds[2].parse::<i32>().unwrap();
            let sect2_end = sect_bounds[3].parse::<i32>().unwrap();

            let sect1 = sect1_start..=sect1_end;
            let sect2 = sect2_start..=sect2_end;

            let is_sect1_contained = is_contained(sect1, sect2_start, sect2_end);
            let is_sect2_contained = is_contained(sect2, sect1_start, sect1_end);

            (is_sect1_contained || is_sect2_contained) as i32
        })
        .sum::<i32>();

    sum
}
