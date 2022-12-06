use std::fs::read_to_string;

fn start_of_packet(data: &str) -> usize {
    let chars: Vec<_> = data.chars().collect();
    chars
        .windows(4)
        .enumerate()
        .find(|(_index, window)| {
            !window[1..4].contains(&window[0])
                && !window[0..3].contains(&window[3])
                && window[1] != window[2]
        })
        .map(|(index, _window)| index)
        .unwrap()
        + 4
}

fn main() {
    let input = read_to_string("input").unwrap();

    let start = start_of_packet(&input);
    println!("Start of packet: {start}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
