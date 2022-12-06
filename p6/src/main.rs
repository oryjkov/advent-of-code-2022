use std::fs;
fn has_dups(s: &[u8]) -> bool {
    for i in 0..s.len() - 1 {
        for j in i + 1..s.len() {
            if s[i] == s[j] {
                return true;
            }
        }
    }
    return false;
}
fn find_start(s: &str, w: usize) -> usize {
    for i in w..s.len() {
        if !has_dups(&s.as_bytes()[i - w..i]) {
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_4() {
        let tcs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        tcs.iter()
            .map(|tc| {
                assert_eq!(find_start(tc.0, 4), tc.1);
            })
            .count();
    }
    #[test]
    fn test_find_14() {
        let tcs = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        tcs.iter()
            .map(|tc| {
                assert_eq!(find_start(tc.0, 14), tc.1);
            })
            .count();
    }
}
fn main() {
    let f = "input.txt";
    let s = fs::read_to_string(f).expect("read fail");

    println!("part 1: {}", find_start(&s, 4));
    println!("part 2: {}", find_start(&s, 14));
}
