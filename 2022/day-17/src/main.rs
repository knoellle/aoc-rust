#![feature(iter_intersperse)]
use std::{
    collections::HashSet,
    fmt::{Display, Write},
    fs::read_to_string,
    ops::{Add, AddAssign},
    thread::sleep,
    time::Duration,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position(isize, isize);

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn offsets(self) -> Position {
        match self {
            Direction::Left => Position(-1, 0),
            Direction::Right => Position(1, 0),
            Direction::Down => Position(0, -1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            c => panic!("Invalid character: {c}"),
        }
    }
}

struct Rock;

impl Rock {
    fn dash() -> &'static [Position] {
        &[
            Position(0, 0),
            Position(1, 0),
            Position(2, 0),
            Position(3, 0),
        ]
    }

    fn plus() -> &'static [Position] {
        &[
            Position(1, 0),
            Position(0, 1),
            Position(1, 1),
            Position(2, 1),
            Position(1, 2),
        ]
    }

    fn l() -> &'static [Position] {
        &[
            Position(0, 0),
            Position(1, 0),
            Position(2, 0),
            Position(2, 1),
            Position(2, 2),
        ]
    }

    fn bar() -> &'static [Position] {
        &[
            Position(0, 0),
            Position(0, 1),
            Position(0, 2),
            Position(0, 3),
        ]
    }

    fn square() -> &'static [Position] {
        &[
            Position(0, 0),
            Position(0, 1),
            Position(1, 0),
            Position(1, 1),
        ]
    }
}

struct Map {
    blocked: HashSet<Position>,
    rock: HashSet<Position>,
    width: isize,
}

impl Map {
    fn new(width: isize) -> Self {
        let blocked = HashSet::from_iter((0..width).map(|x| Position(x, 0)));
        let rock = HashSet::new();
        Self {
            blocked,
            rock,
            width,
        }
    }

    fn part_1<I>(&mut self, jets: I) -> isize
    where
        I: Iterator<Item = Direction> + Clone,
    {
        let mut jets = jets.cycle().intersperse(Direction::Down);
        let mut rocks = [
            Rock::dash(),
            Rock::plus(),
            Rock::l(),
            Rock::bar(),
            Rock::square(),
        ]
        .into_iter()
        .cycle();

        let mut top = 0;
        for _rock_index in 0..25 {
            let mut jets = jets.by_ref();
            self.rock = rocks
                .next()
                .unwrap()
                .iter()
                .map(|position| *position + Position(2, top + 4))
                .collect();
            #[cfg(not(test))]
            println!("{self}");
            loop {
                let direction = jets.next().unwrap();
                let new_rock: HashSet<Position> = self
                    .rock
                    .iter()
                    .map(|position| *position + direction.offsets())
                    .collect();
                if new_rock
                    .iter()
                    .any(|position| self.blocked.contains(position))
                {
                    if direction == Direction::Down {
                        break;
                    }
                } else if new_rock
                    .iter()
                    .all(|position| (0..self.width).contains(&position.0))
                {
                    self.rock = new_rock;
                }
                #[cfg(not(test))]
                {
                    println!("{self}");
                    sleep(Duration::from_millis(10));
                }
            }
            top = self.rock.iter().map(|position| position.1).max().unwrap();
            self.blocked.extend(self.rock.drain());
        }
        #[cfg(not(test))]
        {
            println!("{self}");
            sleep(Duration::from_millis(10));
        }

        // self.blocked
        //     .iter()
        //     .max_by_key(|position| position.1)
        //     .unwrap()
        //     .1
        top
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let top = self
            .blocked
            .iter()
            .chain(self.rock.iter())
            .map(|position| position.1)
            .max()
            .unwrap();

        for y in (0..=top).rev() {
            f.write_char('|')?;
            for x in 0..self.width {
                let c = if self.blocked.contains(&Position(x, y)) {
                    '#'
                } else if self.rock.contains(&Position(x, y)) {
                    '@'
                } else {
                    '.'
                };
                f.write_char(c)?;
            }
            f.write_str("|\n")?;
        }

        Ok(())
    }
}

fn main() {
    let input = read_to_string("example").unwrap();
    let jets: Vec<_> = input.trim().chars().map(Direction::from).collect();
    let mut map = Map::new(7);
    let height = map.part_1(jets.into_iter());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();
        let jets: Vec<_> = input.trim().chars().map(Direction::from).collect();
        let mut map = Map::new(7);
        let height = map.part_1(jets.into_iter());
        assert_eq!(height, 3068);
    }
}
