use std::{collections::BTreeMap, fs::read_to_string, path::PathBuf};

pub fn solve() -> u32 {
    let input = read_to_string("./src/day7/_input.txt").expect("should read input");

    let dir_max_size: u32 = 100_000;
    let mut dir_map: BTreeMap<PathBuf, u32> = BTreeMap::new();
    let mut current_path = PathBuf::new();

    for line in input.lines() {
        let line_parts = line.split_whitespace().collect::<Vec<_>>();

        match (
            line_parts.get(0).copied(),
            line_parts.get(1).copied(),
            line_parts.get(2).copied(),
        ) {
            (Some("$"), Some("cd"), Some("..")) => drop(current_path.pop()),
            (Some("$"), Some("cd"), Some(path)) => current_path.push(path),
            (Some("$"), _, _) => (),
            (Some("dir"), _, _) => (),
            (Some(file_size_str), Some(file_name), _) => {
                let file_size = file_size_str.parse::<u32>().unwrap();
                let file_path = current_path.join(file_name);

                for dir in file_path.parent().unwrap().ancestors() {
                    let dir_path = dir.to_path_buf();

                    dir_map
                        .entry(dir_path)
                        .and_modify(|dir_size| *dir_size += file_size)
                        .or_insert(file_size);
                }
            }
            _ => (),
        }
    }

    dir_map
        .iter()
        .filter_map(|(_, dir_size)| (*dir_size < dir_max_size).then_some(dir_size))
        .sum::<u32>()
}
