use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

impl Symbol {
    fn score(self) -> u32 {
        match self {
            Symbol::Rock => 1,
            Symbol::Paper => 2,
            Symbol::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'X' | 'A' => Symbol::Rock,
            'Y' | 'B' => Symbol::Paper,
            'Z' | 'C' => Symbol::Scissors,
            _ => return Err(value),
        })
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

fn play(opponent: Symbol, player: Symbol) -> Outcome {
    match (opponent, player) {
        (a, b) if a == b => Outcome::Draw,
        (Symbol::Rock, Symbol::Paper)
        | (Symbol::Paper, Symbol::Scissors)
        | (Symbol::Scissors, Symbol::Rock) => Outcome::Win,
        _ => Outcome::Loss,
    }
}

fn main() {
    let input = read_to_string("input").unwrap();

    let games = input.lines();

    let scores = games.map(|game| {
        let opponent = game.chars().nth(0).unwrap().try_into().unwrap();
        let player = game.chars().nth(2).unwrap().try_into().unwrap();
        let outcome = play(opponent, player);

        player.score() + outcome.score()
    });

    println!("Total score: {}", scores.sum::<u32>());
}
