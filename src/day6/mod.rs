use std::fs::read_to_string;
use std::collections::HashSet;

pub fn solve() -> usize {
    let input = read_to_string("./src/day6/_input.txt").expect("should read input");

    let marker_size = 4;
    let mut chars_processed = 0;

    for char_pos in marker_size..=input.len() {
        let marker_start = char_pos - marker_size;
        let marker_end = char_pos;

        let marker = &input[marker_start..marker_end];
        let marker_set: HashSet<char> = marker.chars().collect();

        if marker_set.len() == marker_size {
            chars_processed = char_pos;
            break;
        }
    }
    
    chars_processed
}
