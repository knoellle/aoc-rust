use std::{collections::HashSet, fs::read_to_string};

fn start_of_packet(data: &str, window_size: usize) -> usize {
    let chars: Vec<_> = data.chars().collect();
    chars
        .windows(window_size)
        .enumerate()
        .find(|(_index, window)| HashSet::<_>::from_iter(window.iter()).len() == window_size)
        .map(|(index, _window)| index)
        .unwrap()
        + window_size
}

fn main() {
    let input = read_to_string("input").unwrap();

    let start = start_of_packet(&input, 4);
    println!("Start of packet part 1: {start}");

    let start = start_of_packet(&input, 14);
    println!("Start of packet part 2: {start}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_part_1() {
        let ws = 4;
        assert_eq!(start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb", ws), 7);
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", ws), 5);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg", ws), 6);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", ws), 10);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", ws), 11);
    }

    #[test]
    fn examples_part_2() {
        let ws = 14;
        assert_eq!(start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb", ws), 19);
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", ws), 23);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg", ws), 23);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", ws), 29);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", ws), 26);
    }
}
