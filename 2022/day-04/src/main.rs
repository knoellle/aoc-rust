use std::{fs::read_to_string, ops::RangeInclusive};

fn parse_range(range: &str) -> RangeInclusive<u32> {
    let mut pieces = range.split('-');

    let start = pieces.next().unwrap().parse().unwrap();
    let end = pieces.next().unwrap().parse().unwrap();

    start..=end
}

fn parse_assignment_pairs(input: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let mut assignments = input.split(',');

    (
        parse_range(assignments.next().unwrap()),
        parse_range(assignments.next().unwrap()),
    )
}

fn range_contains_other(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    let a_contains_b = a.contains(b.start()) && a.contains(b.end());
    let b_contains_a = b.contains(a.start()) && b.contains(a.end());

    a_contains_b || b_contains_a
}

fn ranges_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    let a_contains_b = a.contains(b.start()) || a.contains(b.end());
    let b_contains_a = b.contains(a.start()) || b.contains(a.end());

    a_contains_b || b_contains_a
}

fn main() {
    let input = read_to_string("input").unwrap();

    let overlaps = input
        .lines()
        .map(parse_assignment_pairs)
        .filter(|assignments| range_contains_other(&assignments.0, &assignments.1))
        .count();

    println!("Task 1 overlaps: {overlaps}");

    let overlaps = input
        .lines()
        .map(parse_assignment_pairs)
        .filter(|assignments| ranges_overlap(&assignments.0, &assignments.1))
        .count();
    println!("Task 2 overlaps: {overlaps}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range_parsing() {
        assert_eq!(parse_range("0-1"), 0..=1);
        assert_eq!(parse_range("25-103"), 25..=103);
        assert_eq!(parse_range("0-0"), 0..=0);
        assert_eq!(parse_range("100-100"), 100..=100);
    }
}
