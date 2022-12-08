use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
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
    fn x(self, length: isize) -> isize {
        match self {
            Direction::Up | Direction::Down => 0,
            Direction::Left => -length,
            Direction::Right => length,
        }
    }
    fn y(self, length: isize) -> isize {
        match self {
            Direction::Up => -length,
            Direction::Down => length,
            Direction::Left | Direction::Right => 0,
        }
    }
}

struct Map {
    map: Vec<Vec<u32>>,
}

impl Map {
    fn at(&self, x: isize, y: isize) -> Option<u32> {
        if x < 0 || y < 0 {
            return None;
        }
        self.map.get(y as usize)?.get(x as usize).copied()
    }

    fn visible(&self, x: usize, y: usize) -> Option<bool> {
        let height = self.at(x as isize, y as isize)?;
        Some(Direction::all().iter().any(|direction| {
            (1..)
                .map_while(|step| {
                    self.at(
                        x as isize + direction.x(step),
                        y as isize + direction.y(step),
                    )
                })
                .max()
                .map(|max| max < height)
                .unwrap_or(true)
        }))
    }

    #[cfg(test)]
    fn display(&self) {
        self.map.iter().for_each(|line| {
            line.iter().for_each(|height| print!("{height}"));
            println!();
        })
    }

    fn count_visible(&self) -> usize {
        (0..self.map.len())
            .map(|y| {
                (0..self.map[y].len())
                    .filter(|x| self.visible(*x, y).unwrap())
                    .count()
            })
            .sum()
    }

    fn scenic_score(&self, x: usize, y: usize) -> Option<usize> {
        let height = self.at(x as isize, y as isize)?;
        Some(
            Direction::all()
                .iter()
                .map(|direction| {
                    let iter = (1..).map_while(|step| {
                        self.at(
                            x as isize + direction.x(step),
                            y as isize + direction.y(step),
                        )
                    });
                    let max = iter.clone().max().unwrap_or(0);
                    let count = iter.take_while(|x| *x < height).count();

                    if max >= height {
                        count + 1
                    } else {
                        count
                    }
                })
                .product(),
        )
    }

    fn best_scenic_score(&self) -> Option<usize> {
        (0..self.map.len())
            .flat_map(|y| {
                (0..self.map[y].len())
                    .map(|x| self.scenic_score(x, y).unwrap())
                    .max()
            })
            .max()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let map = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|character| {
                        character
                            .to_digit(10)
                            .expect("failed to parse char as digit: {character}")
                    })
                    .collect()
            })
            .collect();
        Map { map }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();

    let map = Map::from(input.as_str());
    println!("Visible trees: {}", map.count_visible());
    println!(
        "Best scenic score: {}",
        map.best_scenic_score().expect("map empty?")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();

        let map = Map::from(input.as_str());
        map.display();
        assert_eq!(map.at(2, 2), Some(3));

        // first row
        assert_eq!(map.visible(1, 1), Some(true));
        assert_eq!(map.visible(2, 1), Some(true));
        assert_eq!(map.visible(3, 1), Some(false));

        // second row
        assert_eq!(map.visible(1, 2), Some(true));
        assert_eq!(map.visible(2, 2), Some(false));
        assert_eq!(map.visible(3, 2), Some(true));

        // third row
        assert_eq!(map.visible(1, 3), Some(false));
        assert_eq!(map.visible(2, 3), Some(true));
        assert_eq!(map.visible(3, 3), Some(false));

        assert_eq!(map.count_visible(), 21);

        assert_eq!(map.scenic_score(2, 1), Some(4));
        assert_eq!(map.scenic_score(2, 3), Some(8));
    }
}
