use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();

    let elves = input.split("\n\n");

    let mut sums: Vec<u32> = elves
        .map(|elf| {
            elf.split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    sums.sort_unstable();

    let best_elf = sums.iter().rev().take(1).sum::<u32>();
    println!("Best: {best_elf}");

    let three_best_elves = sums.iter().rev().take(3).sum::<u32>();
    println!("Sum of three best: {three_best_elves}");
}
