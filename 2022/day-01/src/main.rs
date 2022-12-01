use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();

    let elves = input.split("\n\n");

    let sums = elves.map(|elf| {
        elf.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u32>().unwrap())
            .sum::<u32>()
    });

    let largest_sum = sums.max().unwrap();
    println!("Result: {largest_sum}");
}
