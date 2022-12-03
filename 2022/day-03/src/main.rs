use std::{
    collections::{hash_map::RandomState, HashSet},
    fs::read_to_string,
};

fn shared_items(backpack: &str) -> Vec<char> {
    let pieces = backpack.split_at(backpack.len() / 2);
    let compartment1: HashSet<_, RandomState> = HashSet::from_iter(pieces.0.chars());
    let compartment2 = HashSet::from_iter(pieces.1.chars());

    let output = compartment1.intersection(&compartment2).cloned().collect();
    println!("{:?} {:?} -> {:?}", compartment1, compartment2, output);
    output
}

fn score(item: char) -> u32 {
    let ordinal = item as u32;
    println!("{item}: {ordinal}");
    match item {
        'a'..='z' => ordinal - 'a' as u32 + 1,
        'A'..='Z' => ordinal - 'A' as u32 + 27,
        _ => panic!(),
    }
}

fn main() {
    let input = read_to_string("input").unwrap();

    let backpacks = input.lines();
    let scores = backpacks
        .map(shared_items)
        .flat_map(|shared| shared.into_iter().map(score));

    println!("Part 1: {}", scores.sum::<u32>());
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
