use std::fs;

fn main() {
    // Vec of char tuple that represents how a game of rock-paper-scissor will be played out.
    let raw_games = process_input();

    // Solve part one
    part_one(&raw_games);

    // Solve part two
    part_two(&raw_games);
}

fn part_one(raw_games: &Vec<(char, char)>) {
    // Turn char tuples into Games
    let games: Vec<Game> = raw_games
        .iter()
        .map(|raw_game| create_game(raw_game))
        .collect();

    let mut total_score = 0;

    // For each game check the opponents choice and then my choice
    // Read notes.txt for point calculation
    for game in games {
        match game.opponent {
            OpponentChoice::Rock => match game.me {
                MyChoice::Rock => total_score += 4,
                MyChoice::Paper => total_score += 8,
                MyChoice::Scissor => total_score += 3,
            },
            OpponentChoice::Paper => match game.me {
                MyChoice::Rock => total_score += 1,
                MyChoice::Paper => total_score += 5,
                MyChoice::Scissor => total_score += 9,
            },
            OpponentChoice::Scissor => match game.me {
                MyChoice::Rock => total_score += 7,
                MyChoice::Paper => total_score += 2,
                MyChoice::Scissor => total_score += 6,
            },
        }
    }

    println!("\nAnswer part one");
    println!(
        "\t--> The total score following the strategy guide would be {}\n",
        total_score
    );
}

fn part_two(raw_games: &Vec<(char, char)>) {
    // Turn char tuples into Games
    let games: Vec<Game2> = raw_games
        .iter()
        .map(|raw_game| create_game2(raw_game))
        .collect();

    let mut total_score = 0;

    // For each game check the opponents choice and then the wanted outcome
    for game in games {
        match game.opponent {
            // Loss = 0, Draw = 3, Victory = 6
            OpponentChoice::Rock => match game.outcome {
                // My play is Scissor (+2)
                Outcome::Loss => total_score += 3,
                // My play is Rock (+1)
                Outcome::Draw => total_score += 4,
                // My play is Paper (+2)
                Outcome::Victory => total_score += 8,
            },
            OpponentChoice::Paper => match game.outcome {
                // My play is Rock (+1)
                Outcome::Loss => total_score += 1,
                // My play is Paper (+2)
                Outcome::Draw => total_score += 5,
                // My play is Scissor (+3)
                Outcome::Victory => total_score += 9,
            },
            OpponentChoice::Scissor => match game.outcome {
                // My play is Paper (+2)
                Outcome::Loss => total_score += 2,
                // My play is Scissor (+3)
                Outcome::Draw => total_score += 6,
                // My play is Rock (+1)
                Outcome::Victory => total_score += 7,
            },
        }
    }

    println!("\nAnswer part one");
    println!(
        "\t--> The total score following the strategy guide would be {}\n",
        total_score
    );
}

fn process_input() -> Vec<(char, char)> {
    // Get input as String
    let raw_input = fs::read_to_string("inputs/input-d2.txt").expect("Couldn't read file!");
    // println!("{}", raw_input);

    // Seperate into individual lines. These are the rounds of rock-paper-scissor
    let lines = raw_input.split("\n").collect::<Vec<&str>>();

    // Each round is written as "opponent choice space my choice".
    // Taking the first character (index 0) is the enemy choice
    // and taking the third character (index 2) is my choice.
    // This will result in a tuple of chars for each game of rock-paper-scissor.
    let raw_games: Vec<(char, char)> = lines
        .iter()
        .map(|line| {
            let chars = (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
            chars
        })
        .collect();

    raw_games
}

fn create_game(chars: &(char, char)) -> Game {
    Game {
        opponent: match chars.0 {
            'A' => OpponentChoice::Rock,
            'B' => OpponentChoice::Paper,
            'C' => OpponentChoice::Scissor,
            err => {
                panic!("Unrecognized input: {}", err)
            }
        },
        me: match chars.1 {
            'X' => MyChoice::Rock,
            'Y' => MyChoice::Paper,
            'Z' => MyChoice::Scissor,
            err => {
                panic!("Unrecognized input: {}", err)
            }
        },
    }
}

fn create_game2(chars: &(char, char)) -> Game2 {
    Game2 {
        opponent: match chars.0 {
            'A' => OpponentChoice::Rock,
            'B' => OpponentChoice::Paper,
            'C' => OpponentChoice::Scissor,
            err => {
                panic!("Unrecognized input: {}", err)
            }
        },
        outcome: match chars.1 {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Victory,
            err => {
                panic!("Unrecognized input: {}", err)
            }
        },
    }
}

struct Game {
    opponent: OpponentChoice,
    me: MyChoice,
}

struct Game2 {
    opponent: OpponentChoice,
    outcome: Outcome,
}

enum OpponentChoice {
    Rock,
    Paper,
    Scissor,
}

enum MyChoice {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Loss,
    Draw,
    Victory,
}
