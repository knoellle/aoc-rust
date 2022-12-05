use std::{fs::read_to_string, str::FromStr};

use itertools::Itertools;

fn parse_stacks<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.collect();

    let mut stacks = Vec::new();

    for line in lines.iter().rev().skip(1) {
        let chars: Vec<char> = line.chars().chain(Some(' ')).collect();
        let chunks = chars.chunks(4);
        for (index, chunk) in chunks.enumerate() {
            if stacks.len() <= index {
                stacks.push(Vec::new());
            }
            if chunk[1] != ' ' {
                stacks[index].push(chunk[1])
            }
        }
    }

    stacks
}

#[derive(Debug)]
struct Command {
    count: usize,
    source: usize,
    destination: usize,
}

fn parse_commands<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Command> {
    input
        .map(|line| {
            let mut words = line.split(' ');

            words.next();
            let count = words.next().unwrap().parse().unwrap();
            words.next();
            let source = words.next().unwrap().parse().unwrap();
            words.next();
            let destination = words.next().unwrap().parse().unwrap();

            Command {
                count,
                source,
                destination,
            }
        })
        .collect()
}

fn task_1(mut stacks: Vec<Vec<char>>, commands: Vec<Command>) -> String {
    for command in commands {
        for _ in 0..command.count {
            let item = stacks[command.source - 1].pop().unwrap();
            stacks[command.destination - 1].push(item);
        }
    }
    stacks
        .iter()
        .for_each(|stack| println!("{}", String::from_iter(stack.iter())));

    String::from_iter(stacks.iter().map(|stack| stack.last().unwrap()))
}

fn main() {
    let input = read_to_string("input").unwrap();

    let mut lines = input.lines();

    let stacks = parse_stacks(lines.by_ref().take_while(|line| !line.is_empty()));

    stacks
        .iter()
        .for_each(|stack| println!("{}", String::from_iter(stack.iter())));
    println!();
    let commands = parse_commands(lines.by_ref());

    let solution = task_1(stacks, commands);
    println!();
    println!("Task 1: {solution}")
}