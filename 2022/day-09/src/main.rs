use std::{
    collections::HashSet,
    fs::read_to_string,
    iter::{once, repeat},
};

type Position = (isize, isize);

fn parse_steps(input: &str) -> Vec<Position> {
    input
        .lines()
        .flat_map(|line| {
            let mut pieces = line.split_whitespace();
            let direction = pieces.next().unwrap();
            let length: usize = pieces.next().unwrap().parse().unwrap();
            repeat(match direction {
                "R" => (1, 0),
                "L" => (-1, 0),
                "D" => (0, 1),
                "U" => (0, -1),
                x => panic!("Invalid direction: {x}"),
            })
            .take(length)
        })
        .collect()
}

fn apply_head_steps(steps: &[Position]) -> Vec<Position> {
    once((0, 0))
        .chain(steps.iter().scan((0, 0), |position, step| {
            position.0 += step.0;
            position.1 += step.1;
            Some(*position)
        }))
        .collect()
}

fn chase_head(positions: &[Position]) -> Vec<Position> {
    once((0, 0))
        .chain(positions.iter().scan((0, 0), |tail, head| {
            println!("{head:?} {tail:?}");
            match (head.0 - tail.0, head.1 - tail.1) {
                (x @ 2 | x @ -2, _) => {
                    tail.0 += x / 2;
                    tail.1 = head.1;
                }
                (_, y @ 2 | y @ -2) => {
                    tail.0 = head.0;
                    tail.1 += y / 2;
                }
                (x, y) if x.abs() > 2 || y.abs() > 2 => panic!("Invalid offset: {x} {y}"),
                _ => {}
            }
            Some(*tail)
        }))
        .collect()
}

fn count_unique(positions: &[Position]) -> usize {
    positions.iter().collect::<HashSet<&Position>>().len()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let steps = parse_steps(&input);

    let positions = apply_head_steps(&steps);
    let tail_positions = chase_head(&positions);
    println!("Unique positions: {}", count_unique(&tail_positions));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = read_to_string("example").unwrap();
        let steps = parse_steps(&input);

        let positions = apply_head_steps(&steps);
        let tail_positions = chase_head(&positions);
        println!("Unique positions: {}", count_unique(&tail_positions));
        assert_eq!(count_unique(&tail_positions), 13);
    }
}
