use std::fs::read_to_string;

#[derive(Debug)]
struct Instruction {
    delay: u8,
    value: i32,
    completed: bool,
}

#[derive(Debug)]
struct CPU {
    cycle: i32,
    register: i32,
    instructions: Vec<Instruction>,
}

impl CPU {
    fn tick(&mut self) {
        self.cycle += 1;

        if let Some(instruction) = self.instructions.last_mut() {
            if instruction.delay == 0 {
                self.register += instruction.value;
                instruction.completed = true;
            } else {
                instruction.delay -= 1;
            }
        }

        self.instructions
            .retain(|instruction| instruction.completed == false);
    }
}

pub fn solve() -> (i32, String) {
    let input = read_to_string("./src/day10/_input.txt").expect("should read input");

    let mut cpu = CPU {
        cycle: 1,
        register: 1,
        instructions: vec![],
    };

    let mut signal_strengths: Vec<i32> = vec![];
    let mut canvas: String = String::from("");

    for line in input.lines() {
        let mut line_parts = line.split_whitespace();

        let instruction = Instruction {
            delay: match line_parts.next() {
                Some("addx") => 1,
                _ => 0,
            },
            value: match line_parts.next() {
                Some(num) => num.parse::<i32>().unwrap(),
                _ => 0,
            },
            completed: false,
        };

        cpu.instructions.insert(0, instruction);
    }

    while cpu.instructions.len() > 0 {
        let row = (cpu.cycle - 1) / 40;
        let column = (cpu.cycle - 1) % 40;

        let sprite_start = column - 1;
        let sprite_end = column + 1;
        let sprite = sprite_start..=sprite_end;

        let pixel = if sprite.contains(&cpu.register) {
            '#'
        } else {
            '.'
        };

        if row != 0 && column == 0 {
            canvas.push('\n');
        };

        canvas.push(pixel);

        cpu.tick();

        match cpu.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => signal_strengths.push(cpu.cycle * cpu.register),
            _ => (),
        }
    }

    (signal_strengths.iter().sum(), canvas)
}
