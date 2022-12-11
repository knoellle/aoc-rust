use std::{borrow::Borrow, fs::read_to_string, iter::once};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl From<&str> for Instruction {
    fn from(text: &str) -> Self {
        let mut words = text.split_whitespace();
        match (words.next().unwrap(), words.next()) {
            ("noop", None) => Instruction::Noop,
            ("addx", Some(value)) => Instruction::AddX(value.parse().unwrap()),
            _ => panic!("Invalid instruction: '{text}'"),
        }
    }
}

fn run(instructions: impl Iterator<Item = Instruction>) -> Vec<i32> {
    let mut outputs = Vec::new();
    outputs.push(0); // dummy value: "during the 0th cycle"

    let mut x = 1;
    for instruction in instructions {
        outputs.push(x);
        // println!("")
        if let Instruction::AddX(value) = instruction {
            outputs.push(x);
            x += value
        }
    }

    outputs
}

fn signal_strengths(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let x = (index as i32) * value;
            if [9, 0, 1].contains(&(index % 10)) {
                println!("{index} {value} = {x}",);
            }
            x
        })
        .collect()
}

fn task_1(values: &[i32]) -> i32 {
    let indices = [20, 60, 100, 140, 180, 220];
    let values = indices.map(|index| values[index]);
    values.iter().sum()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let instructions = input.lines().map(|line| line.into());
    let outputs = run(instructions);
    let signal_strengths = signal_strengths(&outputs);
    println!("Task 1: {}", task_1(&signal_strengths));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_task1() {
        let input = read_to_string("example").unwrap();
        let instructions = input.lines().map(|line| line.into());
        let outputs = run(instructions);
        assert_eq!(outputs[1], 1);
        assert_eq!(outputs[2], 1);
        assert_eq!(outputs[3], 16);
        assert_eq!(outputs[4], 16);
        assert_eq!(outputs[5], 5);

        assert_eq!(outputs[20], 21);

        let signal_strengths = signal_strengths(&outputs);

        assert_eq!(signal_strengths[20], 420);
        assert_eq!(signal_strengths[60], 1140);
        assert_eq!(signal_strengths[100], 1800);
        assert_eq!(signal_strengths[140], 2940);
        assert_eq!(signal_strengths[180], 2880);
        assert_eq!(signal_strengths[220], 3960);

        assert_eq!(task_1(&signal_strengths), 13140);
    }
}
