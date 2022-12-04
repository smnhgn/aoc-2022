use std::fs::read_to_string;

pub fn solve() -> i32 {
    let input = read_to_string("day1/input.txt").expect("should read input");
    let mut calories_sum_max = 0;

    for calories_by_elf in input.split("\n\n") {
        let mut calories_sum = 0;

        for calorie_as_str in calories_by_elf.split("\n") {
            let calorie_as_num = calorie_as_str.parse::<i32>().expect("should be number");

            calories_sum = calories_sum + calorie_as_num;
        }

        if calories_sum > calories_sum_max {
            calories_sum_max = calories_sum;
        }
    }

    calories_sum_max
}
