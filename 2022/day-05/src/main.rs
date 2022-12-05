use std::fs::read_to_string;

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

fn get_next_number<'a>(iterator: &mut impl Iterator<Item = &'a str>) -> Option<usize> {
    iterator.next();
    iterator.next()?.parse().ok()
}

fn parse_commands<'a>(input: impl Iterator<Item = &'a str>) -> Option<Vec<Command>> {
    input
        .map(|line| {
            let mut words = line.split(' ');

            let count = get_next_number(&mut words)?;
            let source = get_next_number(&mut words)?;
            let destination = get_next_number(&mut words)?;

            Some(Command {
                count,
                source,
                destination,
            })
        })
        .collect()
}

fn task_1(mut stacks: Vec<Vec<char>>, commands: &Vec<Command>) -> String {
    for command in commands {
        for _ in 0..command.count {
            let item = stacks[command.source - 1].pop().unwrap();
            stacks[command.destination - 1].push(item);
        }
    }

    String::from_iter(stacks.iter().map(|stack| stack.last().unwrap()))
}

fn task_2(mut stacks: Vec<Vec<char>>, commands: &Vec<Command>) -> String {
    for command in commands {
        let mut items = Vec::new();
        for _ in 0..command.count {
            items.push(stacks[command.source - 1].pop().unwrap());
        }
        stacks[command.destination - 1].extend(items.iter().rev());
    }

    String::from_iter(stacks.iter().map(|stack| stack.last().unwrap()))
}

fn main() {
    let input = read_to_string("input").unwrap();

    let mut lines = input.lines();

    let stacks = parse_stacks(lines.by_ref().take_while(|line| !line.is_empty()));
    let commands = parse_commands(lines.by_ref()).unwrap();

    let solution = task_1(stacks.clone(), &commands);
    println!("Task 1: {solution}");

    let solution = task_2(stacks, &commands);
    println!("Task 2: {solution}")
}
