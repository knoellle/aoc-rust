use std::fs::read_to_string;

#[derive(Clone, Copy)]
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

    fn visible(&self, x: isize, y: isize) -> Option<bool> {
        let height = self.at(x, y)?;
        Some(Direction::all().iter().any(|direction| {
            (1..)
                .map_while(|step| self.at(x + direction.x(step), y + direction.y(step)))
                .fuse()
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
                    .filter(|x| self.visible(*x as isize, y as isize).unwrap())
                    .count()
            })
            .sum()
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

        assert_eq!(map.count_visible(), 21)
    }
}
