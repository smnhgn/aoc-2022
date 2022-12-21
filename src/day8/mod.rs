use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

pub fn solve() -> [usize; 2] {
    let input = read_to_string("./src/day8/_input.txt").expect("should read input");

    let mut tree_rows: Vec<Vec<u32>> = vec![];
    let mut tree_columns: Vec<Vec<u32>> = vec![];
    let mut visible_trees: HashSet<Point> = HashSet::new();
    let mut scenic_score_max: usize = 0;

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

            // part two

            let limit_left = left.iter().rev().position(|size| size >= tree_size);
            let visible_left = match limit_left {
                Some(limit) => left.iter().rev().take(limit + 1).count(),
                None => left.iter().count(),
            };

            let limit_right = right.iter().position(|size| size >= tree_size);
            let visible_right = match limit_right {
                Some(limit) => right.iter().take(limit + 1).count(),
                None => right.iter().count(),
            };

            let limit_top = top.iter().rev().position(|size| size >= tree_size);
            let visible_top = match limit_top {
                Some(limit) => top.iter().rev().take(limit + 1).count(),
                None => top.iter().count(),
            };

            let limit_bottom = bottom.iter().position(|size| size >= tree_size);
            let visible_bottom = match limit_bottom {
                Some(limit) => bottom.iter().take(limit + 1).count(),
                None => bottom.iter().count(),
            };

            let scenic_score = visible_left * visible_right * visible_top * visible_bottom;

            scenic_score_max = scenic_score_max.max(scenic_score);
        }
    }

    [visible_trees.len(), scenic_score_max]
}
