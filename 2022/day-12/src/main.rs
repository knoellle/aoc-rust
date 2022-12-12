use std::{collections::VecDeque, fmt::Display, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Cell {
    symbol: char,
    cost: usize,
    best_direction: Option<Direction>,
}

impl From<char> for Cell {
    fn from(symbol: char) -> Self {
        Self {
            symbol,
            cost: usize::MAX,
            best_direction: None,
        }
    }
}

impl Cell {
    fn elevation(&self) -> u8 {
        (match self.symbol {
            'S' => b'a',
            'E' => b'z',
            c => c as u8,
        }) - b'a'
    }

    fn is_start(&self) -> bool {
        self.symbol == 'S'
    }

    fn is_end(&self) -> bool {
        self.symbol == 'E'
    }

    fn new_symbol(&self) -> char {
        match self.best_direction {
            Some(Direction::Up) => '^',
            Some(Direction::Down) => 'V',
            Some(Direction::Left) => '<',
            Some(Direction::Right) => '>',
            None => self.symbol,
        }
    }
}

struct Map {
    width: usize,
    cells: Vec<Cell>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let cells = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|char| char.into())
            .collect();

        Self { width, cells }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, cell) in self.cells.iter().enumerate() {
            f.write_fmt(format_args!("{}", cell.new_symbol()))?;
            if index % self.width == self.width - 1 {
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

impl Map {
    fn apply_direction(&self, current: usize, direction: Direction) -> Option<usize> {
        match direction {
            #[allow(clippy::unnecessary_lazy_evaluations)]
            Direction::Up => (current >= self.width).then(|| current - self.width),
            Direction::Down => {
                (current + self.width < self.cells.len()).then_some(current + self.width)
            }
            #[allow(clippy::unnecessary_lazy_evaluations)]
            Direction::Left => (current % self.width > 0).then(|| current - 1),
            Direction::Right => (current % self.width < self.width - 1).then_some(current + 1),
        }
    }

    fn solve(&mut self) {
        let mut queue = VecDeque::new();

        let end = self
            .cells
            .iter()
            .enumerate()
            .find(|(_index, cell)| cell.is_end())
            .unwrap()
            .0;
        queue.push_back(end);
        self.cells[end].cost = 0;

        while let Some(index) = queue.pop_front() {
            for direction in Direction::all() {
                if Some(direction) == self.cells[index].best_direction {
                    continue;
                }
                if let Some(neighbor) = self.apply_direction(index, direction) {
                    // println!(
                    //     "{} > {}",
                    //     self.cells[neighbor].cost,
                    //     self.cells[index].cost + 1
                    // );
                    if self.cells[neighbor].cost <= self.cells[index].cost + 1 {
                        continue;
                    }

                    // println!(
                    //     "{} > {}",
                    //     self.cells[neighbor].elevation() + 1,
                    //     self.cells[index].elevation()
                    // );
                    if self.cells[neighbor].elevation() + 1 < self.cells[index].elevation() {
                        continue;
                    }
                    self.cells[neighbor].cost = self.cells[index].cost + 1;
                    self.cells[neighbor].best_direction = Some(direction.reverse());
                    queue.push_back(neighbor);
                }
            }
            // println!("{self}");
        }
    }

    fn count_steps_to_end(&self) -> usize {
        let start = self
            .cells
            .iter()
            .enumerate()
            .find(|(_index, cell)| cell.is_start())
            .unwrap()
            .0;
        self.cells[start].cost
    }

    fn lowest_starting_cost(&self) -> usize {
        self.cells
            .iter()
            .filter(|cell| cell.elevation() == 0)
            .min_by_key(|cell| cell.cost)
            .unwrap()
            .cost
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let mut map = Map::from(input.as_str());
    println!("{map}");
    map.solve();
    println!("{map}");
    println!("Steps to end: {}", map.count_steps_to_end());
    println!("Cheapest start: {}", map.lowest_starting_cost());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();
        let mut map = Map::from(input.as_str());
        println!("{map}");
        map.solve();
        assert_eq!(map.count_steps_to_end(), 31);
    }
}
