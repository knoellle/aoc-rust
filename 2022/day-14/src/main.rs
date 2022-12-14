use std::{
    collections::HashSet, fs::read_to_string, ops::RangeInclusive, thread::sleep, time::Duration,
};

use itertools::Itertools;

type Position = (u32, u32);

fn parse_rock_paths(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|position| {
                    let mut numbers = position.split(',').map(|num| num.parse().unwrap());
                    (numbers.next().unwrap(), numbers.next().unwrap())
                })
                .collect()
        })
        .collect()
}

fn sort_range(range: RangeInclusive<u32>) -> RangeInclusive<u32> {
    if range.start() > range.end() {
        *range.end()..=*range.start()
    } else {
        range
    }
}

fn paths_to_positions(paths: &[Vec<Position>]) -> HashSet<Position> {
    let mut positions = HashSet::new();

    for path in paths {
        for pair in path.windows(2) {
            let x_range = sort_range(pair[0].0..=pair[1].0);
            let y_range = sort_range(pair[0].1..=pair[1].1);
            positions.extend(x_range.cartesian_product(y_range))
        }
    }

    positions
}

fn sand_rest_position(
    start: Position,
    blockers: &HashSet<Position>,
    lowest_y: u32,
) -> Option<Position> {
    let mut position = start;

    while position.1 < lowest_y {
        if !blockers.contains(&(position.0, position.1 + 1)) {
            position.1 += 1;
            continue;
        }
        if !blockers.contains(&(position.0 - 1, position.1 + 1)) {
            position.0 -= 1;
            position.1 += 1;
            continue;
        }
        if !blockers.contains(&(position.0 + 1, position.1 + 1)) {
            position.0 += 1;
            position.1 += 1;
            continue;
        }
        return Some(position);
    }

    None
}

fn part_1(mut blockers: HashSet<Position>) -> Vec<Position> {
    let mut rest_positions = Vec::new();
    let lowest_y = blockers.iter().map(|position| position.1).max().unwrap();
    loop {
        let rest_position = sand_rest_position((500, 0), &blockers, lowest_y);
        match rest_position {
            Some(position) => {
                rest_positions.push(position);
                blockers.insert(position);
            }
            None => break,
        }
    }

    rest_positions
}

fn part_2(mut blockers: HashSet<Position>) -> Vec<Position> {
    let mut rest_positions = Vec::new();
    let floor_y = blockers.iter().map(|position| position.1).max().unwrap() + 2;

    blockers.extend((0..1000).map(|x| (x, floor_y)));

    loop {
        let rest_position = sand_rest_position((500, 0), &blockers, floor_y);
        match rest_position {
            Some(position @ (500, 0)) => {
                rest_positions.push(position);
                break;
            }
            Some(position) => {
                rest_positions.push(position);
                blockers.insert(position);
            }
            None => break,
        }
    }

    rest_positions
}

fn main() {
    let input = read_to_string("input").unwrap();
    let paths = parse_rock_paths(&input);
    let blockers = paths_to_positions(&paths);

    let rest_positions = part_1(blockers.clone());
    println!("Pieces of sand that got stuck: {}", rest_positions.len());

    let rest_positions = part_2(blockers);
    println!("Pieces of sand that got stuck: {}", rest_positions.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();
        let paths = parse_rock_paths(&input);
        assert_eq!(
            &paths,
            &[
                [(498, 4), (498, 6), (496, 6)].as_slice(),
                &[(503, 4), (502, 4), (502, 9), (494, 9)]
            ]
        );
        let blockers = paths_to_positions(&paths);
        let rest_positions = part_1(blockers);
        assert_eq!(
            rest_positions[..6],
            [(500, 8), (499, 8), (501, 8), (500, 7), (498, 8), (499, 7)]
        );
    }
}
