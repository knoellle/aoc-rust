use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
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

impl Symbol {
    fn score(self) -> u32 {
        match self {
            Symbol::Rock => 1,
            Symbol::Paper => 2,
            Symbol::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy)]
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

impl TryFrom<char> for Outcome {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => return Err(value),
        })
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

fn symbol_to_play(opponent: Symbol, outcome: Outcome) -> Symbol {
    match (opponent, outcome) {
        (_, Outcome::Draw) => opponent,
        (Symbol::Scissors, Outcome::Win) | (Symbol::Paper, Outcome::Loss) => Symbol::Rock,
        (Symbol::Rock, Outcome::Win) | (Symbol::Scissors, Outcome::Loss) => Symbol::Paper,
        (Symbol::Paper, Outcome::Win) | (Symbol::Rock, Outcome::Loss) => Symbol::Scissors,
    }
}

fn task1_score((opponent, player): (char, char)) -> Result<u32, char> {
    let opponent = opponent.try_into()?;
    let player = player.try_into()?;
    let outcome = play(opponent, player);

    Ok(player.score() + outcome.score())
}

fn task2_score((opponent, outcome): (char, char)) -> Result<u32, char> {
    let opponent = opponent.try_into()?;
    let outcome = outcome.try_into()?;
    let player = symbol_to_play(opponent, outcome);

    Ok(player.score() + outcome.score())
}

fn main() {
    let input = read_to_string("input").unwrap();
    let games = input
        .lines()
        .map(|game| (game.chars().next().unwrap(), game.chars().nth(2).unwrap()));

    let scores = games.clone().flat_map(task1_score);
    println!("Total score (task 1): {}", scores.sum::<u32>());

    let scores = games.flat_map(task2_score);
    println!("Total score (task 2): {}", scores.sum::<u32>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_task_1() {
        assert_eq!(task1_score(('A', 'Y')), Ok(8));
        assert_eq!(task1_score(('B', 'X')), Ok(1));
        assert_eq!(task1_score(('C', 'Z')), Ok(6));
    }

    #[test]
    fn example_task_2() {
        assert_eq!(task2_score(('A', 'Y')), Ok(4));
        assert_eq!(task2_score(('B', 'X')), Ok(1));
        assert_eq!(task2_score(('C', 'Z')), Ok(7));
    }
}
