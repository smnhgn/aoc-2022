use std::fs::read_to_string;

enum Option {
    Rock,
    Paper,
    Scissors,
}

fn get_score(player_option: Option, opponent_option: Option) -> i32 {
    let option_score = match player_option {
        Option::Rock => 1,
        Option::Paper => 2,
        Option::Scissors => 3,
    };

    let game_score = match player_option {
        Option::Rock => match opponent_option {
            Option::Rock => 3,
            Option::Paper => 0,
            Option::Scissors => 6,
        },
        Option::Paper => match opponent_option {
            Option::Rock => 6,
            Option::Paper => 3,
            Option::Scissors => 0,
        },
        Option::Scissors => match opponent_option {
            Option::Rock => 0,
            Option::Paper => 6,
            Option::Scissors => 3,
        },
    };

    option_score + game_score
}

pub fn solve() -> i32 {
    let input = read_to_string("./src/day2/_input.txt").expect("should read input");
    let mut score_total = 0;

    for line in input.split("\n") {
        let round: Vec<&str> = line.split_whitespace().collect();

        assert_eq!(2, round.len());

        let opponent_option = match round[0] {
            "A" => Option::Rock,
            "B" => Option::Paper,
            "C" => Option::Scissors,
            _ => panic!("unknown option for opponent"),
        };
        let player_option = match round[1] {
            "X" => Option::Rock,
            "Y" => Option::Paper,
            "Z" => Option::Scissors,
            _ => panic!("unknown option for player"),
        };

        score_total = score_total + get_score(player_option, opponent_option);
    }

    score_total
}
