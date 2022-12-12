use std::fs;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), _);
    }
    #[test]
    fn test_part2() {
    }
}

fn solve_part1(f: &str) -> i32 {
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .count();
    -1
}

fn solve_part2(f: &str) -> i32 {
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .count();
    -1
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
