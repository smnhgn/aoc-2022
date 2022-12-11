use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    pub fn stay(&mut self) -> &mut Self {
        self
    }
    pub fn up(&mut self) -> &mut Self {
        self.y += 1;
        self
    }
    pub fn down(&mut self) -> &mut Self {
        self.y -= 1;
        self
    }
    pub fn left(&mut self) -> &mut Self {
        self.x -= 1;
        self
    }
    pub fn right(&mut self) -> &mut Self {
        self.x += 1;
        self
    }
    pub fn follow(&mut self, head: &Point) -> &mut Self {
        let distance_x = head.x - self.x;
        let distance_y = head.y - self.y;

        match [distance_x, distance_y] {
            [2, 0] => self.right(),
            [2, 1] => self.right().up(),
            [1, 2] => self.right().up(),
            [0, 2] => self.up(),
            [-1, 2] => self.left().up(),
            [-2, 1] => self.left().up(),
            [-2, 0] => self.left(),
            [-2, -1] => self.left().down(),
            [-1, -2] => self.left().down(),
            [0, -2] => self.down(),
            [1, -2] => self.right().down(),
            [2, -1] => self.right().down(),
            _ => self,
        }
    }
}

pub fn solve() -> usize {
    let input = read_to_string("./src/day9/_input.txt").expect("should read input");

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        let line_parts = line.split_whitespace().collect::<Vec<_>>();
        let direction = line_parts[0];
        let times = line_parts[1].parse::<u16>().unwrap();

        for _ in 0..times {
            match direction {
                "U" => head.up(),
                "D" => head.down(),
                "L" => head.left(),
                "R" => head.right(),
                _ => head.stay(),
            };
            tail.follow(&head);

            visited.insert(tail.clone());
        }
    }

    visited.len()
}
