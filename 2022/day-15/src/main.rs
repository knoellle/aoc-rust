use std::{collections::HashSet, fs::read_to_string};

use eyre::{Context, Result};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
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

    fn range_on_row(&self, y: i64) -> i64 {
        self.clearing_distance - (self.position.1 - y).abs()
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

// too slow
fn find_distress_signal_brute_force(
    sensors: &[Sensor],
    min: Position,
    max: Position,
) -> Option<Position> {
    (min.0..=max.0).into_par_iter().find_map_any(|x| {
        (min.1..=max.1)
            .into_par_iter()
            .find_any(|y| !sensors.iter().any(|sensor| sensor.in_range((x, *y))))
            .map(|y| (x, y))
    })
}

enum Transition {
    Enter,
    Exit,
}

// doesn't work
fn find_distress_signal_events(
    sensors: &[Sensor],
    min: Position,
    max: Position,
) -> Option<Position> {
    for y in min.1..=max.1 {
        let mut events: Vec<_> = sensors
            .iter()
            .flat_map(|sensor| {
                [
                    (
                        sensor.position.0 - sensor.range_on_row(y),
                        Transition::Enter,
                    ),
                    (sensor.position.0 + sensor.range_on_row(y), Transition::Exit),
                ]
            })
            .collect();
        events.sort_by_key(|event| event.0);
        events
            .iter()
            .scan(0, |state, event| {
                *state += match event.1 {
                    Transition::Enter => 1,
                    Transition::Exit => -1,
                };
                (*state == 0).then_some(event)
            })
            .skip_while(|event| event.0 < min.0)
            .take_while(|event| event.0 <= max.0)
            .next()
            .map(|x| (y, x));
    }

    None
}

// wörks
fn find_distress_signal_borders(
    sensors: &[Sensor],
    min: Position,
    max: Position,
) -> Option<Position> {
    let mut positions = sensors
        .iter()
        // .take(1)
        .flat_map(|sensor| {
            // println!("range: {}", sensor.clearing_distance + 1);
            [(0, 1, 1, 1), (1, 0, -1, 1), (0, -1, -1, -1), (-1, 0, 1, -1)]
                .iter()
                .flat_map(|border| {
                    (0..sensor.clearing_distance).map(|i| {
                        (
                            sensor.position.0
                                + border.0 * (sensor.clearing_distance + 1)
                                + i * border.2,
                            sensor.position.1
                                + border.1 * (sensor.clearing_distance + 1)
                                + i * border.3,
                        )
                    })
                })
        })
        // .inspect(|position| println!("{position:?}"))
        .filter(|position| {
            (min.0..=max.0).contains(&position.0) && (min.1..=max.1).contains(&position.1)
        });
    positions.find(|position| !sensors.iter().any(|sensor2| sensor2.in_range(*position)))
}

fn tuning_frequency(position: Position) -> i64 {
    position.0 * 4_000_000 + position.1
}

fn main() {
    let input = read_to_string("input").unwrap();
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|line| Sensor::parse(line).unwrap())
        .collect();

    println!("Part 1: {}", blockers_in_row(&sensors, 2000000));
    let position = find_distress_signal_borders(&sensors, (0, 0), (4_000_000, 4_000_000));
    println!("Part 2: {:?}", position);
    println!("Tuning frequency: {}", tuning_frequency(position.unwrap()));
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
        let position = find_distress_signal_borders(&sensors, (0, 0), (20, 20));
        assert_eq!(position, Some((14, 11)));
        assert_eq!(tuning_frequency(position.unwrap()), 56000011);
    }
}
