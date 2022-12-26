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
            [2, 2] => self.right().up(),
            [1, 2] => self.right().up(),
            [0, 2] => self.up(),
            [-1, 2] => self.left().up(),
            [-2, 2] => self.left().up(),
            [-2, 1] => self.left().up(),
            [-2, 0] => self.left(),
            [-2, -1] => self.left().down(),
            [-2, -2] => self.left().down(),
            [-1, -2] => self.left().down(),
            [0, -2] => self.down(),
            [1, -2] => self.right().down(),
            [2, -2] => self.right().down(),
            [2, -1] => self.right().down(),
            _ => self,
        }
    }
    pub fn go(&mut self, direction: &str) -> &mut Self {
        match direction {
            "U" => self.up(),
            "D" => self.down(),
            "L" => self.left(),
            "R" => self.right(),
            _ => self.stay(),
        };

        self
    }
}

fn get_visited_points_len(rope: &mut [Point], instructions: &Vec<(&str, usize)>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();

    for (direction, times) in instructions {
        for _ in 0..*times {
            rope[0].go(direction);

            for i in 1..rope.len() {
                let prev = rope[i - 1];

                rope[i].follow(&prev);
            }

            let tail = rope.last().unwrap().clone();

            visited.insert(tail);
        }
    }

    visited.len()
}

pub fn solve() -> [usize; 2] {
    let input = read_to_string("./src/day9/_input.txt").expect("should read input");

    let rope_first_part = &mut [Point { x: 0, y: 0 }; 2];
    let rope_second_part = &mut [Point { x: 0, y: 0 }; 10];
    let mut instructions: Vec<(&str, usize)> = vec![];

    for line in input.lines() {
        let line_parts = line.split_whitespace().collect::<Vec<_>>();
        let direction = line_parts[0];
        let times = line_parts[1].parse::<usize>().unwrap();

        instructions.push((direction, times));
    }

    let visited_first_part = get_visited_points_len(rope_first_part, &instructions);
    let visited_second_part = get_visited_points_len(rope_second_part, &instructions);

    [visited_first_part, visited_second_part]
}
