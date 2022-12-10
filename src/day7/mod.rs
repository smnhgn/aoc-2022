use std::{collections::BTreeMap, fs::read_to_string, path::PathBuf};

pub fn solve() -> u32 {
    let input = read_to_string("./src/day7/_input.txt").expect("should read input");

    let dir_max_size: u32 = 100_000;
    let mut dir_map: BTreeMap<PathBuf, u32> = BTreeMap::new();
    let mut current_path = PathBuf::new();

    for line in input.lines() {
        let line_parts = line.split_whitespace().collect::<Vec<_>>();

        match line_parts.as_slice() {
            ["$", "cd", ".."] => drop(current_path.pop()),
            ["$", "cd", path] => current_path.push(path),
            ["$" | "dir", ..] => (), // ignore
            [file_size, ..] => {
                let file_size = file_size.parse::<u32>().unwrap();

                for dir in current_path.ancestors() {
                    dir_map
                        .entry(dir.to_path_buf())
                        .and_modify(|dir_size| *dir_size += file_size)
                        .or_insert(file_size);
                }
            }
            _ => (), // ignore
        }
    }

    dir_map
        .iter()
        .filter_map(|(_, dir_size)| (*dir_size < dir_max_size).then_some(dir_size))
        .sum::<u32>()
}
