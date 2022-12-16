use std::{collections::HashSet, fs::read_to_string, num::ParseIntError};

use eyre::{Context, Result};
use regex::Regex;

type Position = (i64, i64);

fn manhattan(p1: Position, p2: Position) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Position,
    beacon: Position,
    clearing_distance: i64,
}

impl Sensor {
    fn parse(input: &str) -> Result<Self> {
        let regex = Regex::new(r"-?\d+").unwrap();

        let mut matches = regex.find_iter(input);
        let position = (next_number(&mut matches)?, next_number(&mut matches)?);
        let beacon = (next_number(&mut matches)?, next_number(&mut matches)?);
        let clearing_distance = manhattan(position, beacon);

        Ok(Sensor {
            position,
            beacon,
            clearing_distance,
        })
    }

    fn in_range(&self, position: Position) -> bool {
        manhattan(self.position, position) <= self.clearing_distance
    }
}

fn next_number<'a>(iterator: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<i64> {
    iterator
        .next()
        .ok_or(eyre::eyre!("Empty iterator"))?
        .as_str()
        .parse()
        .wrap_err("Failed to parse")
}

fn blockers_in_row(sensors: &[Sensor], y: i64) -> usize {
    let sensors: Vec<Sensor> = sensors
        .iter()
        .filter(|sensor| {
            // println!("{sensor:?}");
            // println!("Y distance: {}", (sensor.position.1 - y).abs());
            (sensor.position.1 - y).abs() <= sensor.clearing_distance
        })
        .inspect(|sensor| println!("{sensor:?}"))
        .cloned()
        .collect();
    let min = sensors
        .iter()
        .map(|sensor| dbg!(sensor.position.0 - sensor.clearing_distance))
        .min()
        .unwrap();
    let max = sensors
        .iter()
        .map(|sensor| dbg!(sensor.position.0 + sensor.clearing_distance))
        .max()
        .unwrap();

    let count = (min..=max)
        .filter(|x| sensors.iter().any(|sensor| sensor.in_range((*x, y))))
        .count();

    let beacons: HashSet<_> = sensors
        .iter()
        .filter_map(|sensor| (sensor.beacon.1 == y).then_some(sensor.beacon.0))
        .collect();

    count - beacons.len()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|line| Sensor::parse(line).unwrap())
        .collect();

    println!("Part 1: {}", blockers_in_row(&sensors, 2000000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();
        let sensors: Vec<Sensor> = input
            .lines()
            .map(|line| Sensor::parse(line).unwrap())
            .inspect(|sensor| println!("{sensor:?}"))
            .collect();
        assert_eq!(blockers_in_row(&sensors, 10), 26);
    }
}
