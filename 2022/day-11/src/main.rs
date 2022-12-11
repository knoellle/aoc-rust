use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
    Double,
}

impl Operation {
    fn apply(&self, old: u32) -> u32 {
        match self {
            Operation::Add(value) => old + value,
            Operation::Multiply(value) => old * value,
            Operation::Square => old * old,
            Operation::Double => old + old,
        }
    }
}

impl From<&str> for Operation {
    fn from(expression: &str) -> Self {
        let mut pieces = expression.split_whitespace();
        assert_eq!(pieces.next(), Some("old"));
        match (pieces.next().unwrap(), pieces.next().unwrap()) {
            ("+", "old") => Operation::Double,
            ("+", value) => Operation::Add(value.parse().unwrap()),
            ("*", "old") => Operation::Square,
            ("*", value) => Operation::Multiply(value.parse().unwrap()),
            _ => panic!("Invalid operation: {expression}"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test_divisor: u32,
    test_targets_true: usize,
    test_targets_false: usize,
}

fn parse_csv_numbers(inputs: &str) -> Vec<u32> {
    inputs.split(", ").map(|num| num.parse().unwrap()).collect()
}

fn get_next_after<'a>(mut lines: impl Iterator<Item = &'a str>, delimiter: &'a str) -> &'a str {
    lines.next().unwrap().split_once(delimiter).unwrap().1
}

impl Monkey {
    fn inspect(&self, item: u32) -> u32 {
        self.operation.apply(item) / 3
    }
    fn throw(&self, item: u32) -> usize {
        if item % self.test_divisor == 0 {
            self.test_targets_true
        } else {
            self.test_targets_false
        }
    }
}

impl<'a, T> From<T> for Monkey
where
    T: Iterator<Item = &'a str>,
{
    fn from(mut lines: T) -> Self {
        let starting_items = parse_csv_numbers(get_next_after(lines.by_ref(), ": "));
        let operation = Operation::from(get_next_after(lines.by_ref(), "new = "));
        let test_divisor = get_next_after(lines.by_ref(), "divisible by ")
            .parse()
            .unwrap();
        let test_targets_true = get_next_after(lines.by_ref(), "throw to monkey ")
            .parse()
            .unwrap();
        let test_targets_false = get_next_after(lines.by_ref(), "throw to monkey ")
            .parse()
            .unwrap();

        Self {
            items: starting_items,
            operation,
            test_divisor,
            test_targets_true,
            test_targets_false,
        }
    }
}

fn play_round(monkeys: &mut [Monkey]) -> Vec<usize> {
    let mut business: Vec<usize> = Vec::new();
    business.resize(monkeys.len(), 0);

    for index in 0..monkeys.len() {
        let items: Vec<u32> = monkeys[index].items.drain(..).collect();
        business[index] += items.len();

        for item in items {
            let new_item = monkeys[index].inspect(item);
            let new_index = monkeys[index].throw(new_item);
            monkeys[new_index].items.push(new_item);
        }
    }

    business
}

fn task_1(monkeys: &mut [Monkey]) -> usize {
    let mut business: Vec<usize> = Vec::new();
    business.resize(monkeys.len(), 0);

    for _round in 0..20 {
        let new_busines = play_round(monkeys);
        business
            .iter_mut()
            .zip(new_busines)
            .for_each(|(old, additional)| *old += additional)
    }

    business.sort();
    business.reverse();
    business[0] * business[1]
}

fn main() {
    let input = read_to_string("input").unwrap();
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| Monkey::from(monkey.lines().skip(1)))
        .collect();
    let business = task_1(&mut monkeys);
    println!("Business score: {business}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();
        let mut monkeys: Vec<Monkey> = input
            .split("\n\n")
            .map(|monkey| Monkey::from(monkey.lines().skip(1)))
            .collect();
        let business = task_1(&mut monkeys);
        assert_eq!(business, 10605);
    }
}
