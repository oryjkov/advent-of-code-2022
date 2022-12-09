use std::{collections::HashSet, fs};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_p1("test.txt"), 13);
    }
}

fn visualize(h: (i32, i32), t: (i32, i32)) {
    let (width, height) = (5, 5);
    for row in (0..height).rev() {
        for col in 0..width {
            let c = if (row, col) == h {
                'H'
            } else if (row, col) == t {
                'T'
            } else if (row, col) == (0, 0) {
                's'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn solve_p1(f: &str) -> usize {
    let mut pos = HashSet::new();
    let mut h = (0i32, 0i32);
    let mut t = (0i32, 0i32);
    pos.insert(t);
    fs::read_to_string(f)
        .expect("read failed")
        .split('\n')
        .filter(|l| l.len() > 0)
        //.take(2)
        .map(|l| {
            let input: Vec<&str> = l.split_whitespace().collect();
            let d = match input[0] {
                "U" => (1, 0),
                "D" => (-1, 0),
                "L" => (0, -1),
                "R" => (0, 1),
                _ => (-1000, 0),
            };
            let n = input[1].parse::<usize>().unwrap();
            for _ in 0..n {
                h.0 += d.0;
                h.1 += d.1;
                let d0 = (t.0 - h.0).abs();
                let d1 = (t.1 - h.1).abs();
                //visualize(h, t);
                if d0 * d0 + d1 * d1 <= 2 {
                    continue;
                }

                t.0 = t.0 + (h.0 - t.0).signum() * d0.min(1);
                t.1 = t.1 + (h.1 - t.1).signum() * d1.min(1);
                pos.insert(t);
            }
        })
        .count();
    pos.len()
}

fn main() {
    println!("part1: {}", solve_p1("input.txt"));
}
