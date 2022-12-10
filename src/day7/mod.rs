use std::fs::read_to_string;

#[cfg(test)]
mod tests;

struct File {
    name: String,
    size: u32,
}

struct Directory {
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
    fn size(self) -> u32 {
        let sum_files: u32 = self.files.into_iter().map(|file| file.size).sum();
        let sum_directories: u32 = self.directories.into_iter().map(|dir| dir.size()).sum();

        sum_files + sum_directories
    }
}

struct Line {
    files: Vec<File>,
    directories: Vec<Directory>,
}

pub fn solve() -> usize {
    let input = read_to_string("./src/day6/_input.txt").expect("should read input");

    1
}
