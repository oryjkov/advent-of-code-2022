use std::fs;
use std::str::from_utf8;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let tests = [
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];
        for (exp, inp) in tests {
            let got = parse_snum(inp.as_bytes());
            println!("testing {}, expected: {}, got: {}", inp, exp, got);
            assert_eq!(got, exp);
        }
        println!("============= REV ========");
        for (inp, exp) in tests {
            let got = format_snum(inp);
            println!(
                "testing {}, expected: {}, got: {}",
                inp,
                exp,
                from_utf8(&got).unwrap()
            );
            assert_eq!(got, exp.as_bytes());
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), "2=-1=0");
        assert_eq!(solve_part1("input.txt"), "2-0-0=1-0=2====20=-2");
    }
}

fn sdigit_to_int(sd: u8) -> i64 {
    match sd {
        b'=' => -2,
        b'-' => -1,
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        _ => panic!("wrong snafu digit"),
    }
}
fn parse_snum(snum: &[u8]) -> i64 {
    snum.iter()
        .map(|sd| sdigit_to_int(*sd))
        .fold(0, |acc, item| acc * 5 + item)
}
fn format_snum(num: i64) -> Vec<u8> {
    let mut rv = vec![];
    let mut n = num;
    let mut carry;
    let mut sd;
    while n > 0 {
        (sd, carry) = match n % 5 {
            0 => (b'0', 0),
            1 => (b'1', 0),
            2 => (b'2', 0),
            3 => (b'=', -1),
            4 => (b'-', -1),
            _ => panic!("unexpected remainder"),
        };
        rv.push(sd);
        n = n / 5;
        n = n - carry;
    }
    rv.into_iter().rev().collect()
}

fn solve_part1(f: &str) -> String {
    let s = fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| parse_snum(l.as_bytes()))
        .sum();
    from_utf8(&format_snum(s)).unwrap().to_string()
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
}
