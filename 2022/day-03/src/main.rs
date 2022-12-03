use std::{
    collections::{hash_map::RandomState, HashSet},
    fs::read_to_string,
};

fn shared_items_backpack(backpack: &str) -> Vec<char> {
    let pieces = backpack.split_at(backpack.len() / 2);
    let compartment1: HashSet<_, RandomState> = HashSet::from_iter(pieces.0.chars());
    let compartment2 = HashSet::from_iter(pieces.1.chars());

    compartment1.intersection(&compartment2).cloned().collect()
}

fn shared_item_elves(elves: &[&str]) -> char {
    let shared = elves
        .iter()
        .map(|backpack| HashSet::from_iter(backpack.chars()))
        .reduce(|a: HashSet<_, RandomState>, b| a.intersection(&b).cloned().collect())
        .expect("No elves? o.O");

    *shared.iter().next().expect("there was no shared item")
}

fn score(item: char) -> u32 {
    let ordinal = item as u32;
    match item {
        'a'..='z' => ordinal - 'a' as u32 + 1,
        'A'..='Z' => ordinal - 'A' as u32 + 27,
        _ => panic!("Invalid character"),
    }
}

fn main() {
    let input = read_to_string("input").unwrap();

    let backpacks = input.lines();
    let scores = backpacks
        .clone()
        .map(shared_items_backpack)
        .flat_map(|shared| shared.into_iter().map(score));

    println!("Part 1: {}", scores.sum::<u32>());

    let backpacks: Vec<_> = backpacks.collect();
    let groups = backpacks.chunks(3);
    let scores = groups.map(shared_item_elves).map(score);

    println!("Part 2: {}", scores.sum::<u32>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_correct() {
        assert_eq!(score('a'), 1);
        assert_eq!(score('z'), 26);
        assert_eq!(score('A'), 27);
        assert_eq!(score('Z'), 52);

        assert_eq!(score('p'), 16);
        assert_eq!(score('L'), 38);
        assert_eq!(score('P'), 42);
        assert_eq!(score('v'), 22);
        assert_eq!(score('t'), 20);
        assert_eq!(score('s'), 19);
    }
}
