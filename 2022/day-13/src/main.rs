use std::{cmp::Ordering, fmt::Write, iter::Peekable};
use std::{fmt::Display, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Value(u32),
    List(Vec<Item>),
}

impl Item {
    fn parse<I>(symbols: &mut Peekable<I>) -> Option<Item>
    where
        I: Iterator<Item = char>,
    {
        match symbols.peek() {
            Some('[') => Item::parse_list(symbols),
            Some('0'..='9') => Item::parse_value(symbols),
            Some(x) => panic!("{x} is not a list or a digit"),
            None => panic!("Empty symbols"),
        }
    }

    fn parse_value<I>(symbols: &mut Peekable<I>) -> Option<Item>
    where
        I: Iterator<Item = char>,
    {
        let mut chars = String::new();
        loop {
            match symbols.peek() {
                Some(symbol) if symbol.is_ascii_digit() => chars.push(symbols.next().unwrap()),
                _ => break,
            }
        }

        Some(Item::Value(chars.parse().unwrap()))
    }

    fn parse_list<I>(symbols: &mut Peekable<I>) -> Option<Item>
    where
        I: Iterator<Item = char>,
    {
        let mut items = Vec::new();

        loop {
            match symbols.next() {
                Some('[') if symbols.peek() == Some(&']') => {
                    assert_eq!(symbols.next(), Some(']'));
                    break;
                }
                Some('[' | ',') => items.push(Item::parse(symbols).unwrap()),
                Some(']') => break,
                Some(x) => panic!("unexpected character: {x}"),
                None => panic!("unexpected end of string"),
            }
        }

        Some(Item::List(items))
    }

    fn compare_list(&self, other_item: &Item) -> Ordering {
        let other = match other_item {
            Item::List(other) => other,
            _ => panic!("non list item passed to compare_list"),
        };
        let result = match self {
            Item::Value(value) => Item::List(vec![Item::Value(*value)]).compare_list(other_item),
            Item::List(list) => list
                .iter()
                .zip(other)
                .map(|(a, b)| a.partial_cmp(b).unwrap())
                .find(|ordering| *ordering != Ordering::Equal)
                .unwrap_or_else(|| list.len().partial_cmp(&other.len()).unwrap()),
        };
        println!("Comparing lists: {self}, {other_item}");
        println!("{result:?}");
        result
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Value(value) => f.write_fmt(format_args!("{value}"))?,
            Item::List(list) => {
                f.write_char('[')?;
                list.iter().enumerate().for_each(|(index, item)| {
                    if index > 0 {
                        f.write_char(',').unwrap();
                    }
                    f.write_fmt(format_args!("{}", item)).unwrap();
                });
                f.write_char(']')?;
            }
        }
        Ok(())
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Value(value), Item::Value(value2)) => value.partial_cmp(value2),
            (Item::Value(_), Item::List(_)) => Some(self.compare_list(other)),
            (Item::List(_), Item::Value(_)) => Some(other.compare_list(self).reverse()),
            (Item::List(_), Item::List(_)) => Some(self.compare_list(other)),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let pairs: Vec<Vec<_>> = input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|line| Item::parse(&mut line.chars().peekable()).unwrap())
                .collect()
        })
        .collect();
    let right_order_indices = pairs.iter().enumerate().filter_map(|(index, pair)| {
        (dbg!(pair[0].partial_cmp(&pair[1]).unwrap()) == Ordering::Less).then_some(index + 1)
    });
    let sum: usize = right_order_indices.sum();
    println!("Sum of correctly ordered pair indices: {sum}");

    let received_packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Item::parse(&mut line.chars().peekable()).unwrap());
    let dividers = vec![
        Item::List(vec![Item::List(vec![Item::Value(2)])]),
        Item::List(vec![Item::List(vec![Item::Value(6)])]),
    ];
    let mut all_packets = dividers.clone();
    all_packets.extend(received_packets);
    all_packets.sort();

    let divider_indices = dividers
        .iter()
        .map(|divider| {
            all_packets
                .iter()
                .enumerate()
                .find(|(_index, packet)| divider == *packet)
                .unwrap()
        })
        .map(|(index, _packet)| index + 1);
    println!("Decoder key: {}", divider_indices.product::<usize>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        let input = read_to_string("input").unwrap();
        for line in input.lines().filter(|line| !line.is_empty()) {
            let item = Item::parse(&mut line.chars().peekable()).unwrap();
            assert_eq!(line, item.to_string());
        }
    }

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();
        let pairs: Vec<Vec<_>> = input
            .split("\n\n")
            .map(|pair| {
                pair.lines()
                    .map(|line| Item::parse(&mut line.chars().peekable()).unwrap())
                    .collect()
            })
            .collect();
        let right_order_indices: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter_map(|(index, pair)| {
                (dbg!(pair[0].partial_cmp(&pair[1]).unwrap()) == Ordering::Less)
                    .then_some(index + 1)
            })
            .collect();
        assert_eq!(right_order_indices, &[1, 2, 4, 6]);

        let sum: usize = right_order_indices.iter().sum();
        assert_eq!(sum, 13);
    }

    #[test]
    fn example_reverse() {
        let input = read_to_string("example").unwrap();
        let pairs: Vec<Vec<_>> = input
            .split("\n\n")
            .map(|pair| {
                pair.lines()
                    .map(|line| Item::parse(&mut line.chars().peekable()).unwrap())
                    .collect()
            })
            .collect();
        let right_order_indices: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter_map(|(index, pair)| {
                (dbg!(pair[1].partial_cmp(&pair[0]).unwrap()) == Ordering::Less)
                    .then_some(index + 1)
            })
            .collect();
        assert_eq!(right_order_indices, &[3, 5, 7, 8]);

        let sum: usize = right_order_indices.iter().sum();
        assert_eq!(sum, 23);
    }

    #[test]
    fn input() {
        let input = read_to_string("input").unwrap();
        let pairs: Vec<(&str, Vec<_>)> = input
            .split("\n\n")
            .map(|pair| {
                (
                    pair,
                    pair.lines()
                        .map(|line| Item::parse(&mut line.chars().peekable()).unwrap())
                        .collect(),
                )
            })
            .collect();
        let right_order_indices = pairs.iter().map(|(original, pair)| {
            dbg!(original, pair[0].partial_cmp(&pair[1]).unwrap()).1 == Ordering::Less
        });
        let output = read_to_string("output").unwrap();
        let expected_output = output.lines().map(|line| line.parse::<bool>().unwrap());
        right_order_indices
            .zip(expected_output)
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }
}
