use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

pub fn solve() -> usize {
    let input = read_to_string("./src/day8/_input.txt").expect("should read input");

    let mut tree_rows: Vec<Vec<u32>> = vec![];
    let mut tree_columns: Vec<Vec<u32>> = vec![];
    let mut visible_trees: HashSet<Point> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        tree_rows.push(vec![]);

        for (x, char) in line.chars().enumerate() {
            if y == 0 {
                tree_columns.push(vec![]);
            }

            let tree = char.to_digit(10).unwrap();

            tree_rows[y].push(tree);
            tree_columns[x].push(tree);
        }
    }

    let y_max = tree_rows.len() - 1;
    let x_max = tree_columns.len() - 1;

    for y in 0..=y_max {
        for x in 0..=x_max {
            let tree_size = &tree_rows[y][x];
            let left = &tree_rows[y][..x];
            let right = &tree_rows[y][x + 1..];
            let top = &tree_columns[x][..y];
            let bottom = &tree_columns[x][y + 1..];
            let min_size = [left, right, top, bottom]
                .iter()
                .filter_map(|side| side.iter().max())
                .min()
                .unwrap();

            if [0, x_max].contains(&x) || [0, y_max].contains(&y) || tree_size > min_size {
                visible_trees.insert(Point { x, y });
            }
        }
    }

    visible_trees.len()
}
