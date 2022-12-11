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
        let trees = line
            .chars()
            .filter_map(|char| char.to_digit(10))
            .collect::<Vec<_>>();

        tree_rows.push(vec![]);

        for (x, tree) in trees.into_iter().enumerate() {
            if y == 0 {
                tree_columns.push(vec![])
            }

            tree_rows[y].push(tree);
            tree_columns[x].push(tree);
        }
    }

    for y in 0..tree_rows.len() {
        for x in 0..tree_columns.len() {
            let tree = tree_rows[y][x];
            let left_max = tree_rows[y][..x].iter().max().unwrap_or(&0);
            let right_max = tree_rows[y][x + 1..].iter().max().unwrap_or(&0);
            let top_max = tree_columns[x][..y].iter().max().unwrap_or(&0);
            let bottom_max = tree_columns[x][y + 1..].iter().max().unwrap_or(&0);

            if y == 0
                || x == 0
                || y == tree_rows.len() - 1
                || x == tree_columns.len() - 1
                || tree > *left_max
                || tree > *right_max
                || tree > *top_max
                || tree > *bottom_max
            {
                visible_trees.insert(Point { x, y });
            }
        }
    }

    visible_trees.len()
}
