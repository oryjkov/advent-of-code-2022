use std::{collections::HashSet, fs};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve::<2>("test.txt"), 13);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve::<10>("test2.txt"), 36);
    }
}

fn visualize(tails: &[(i32, i32)]) {
    let (width, height) = (5, 5);
    for row in (0..height).rev() {
        for col in 0..width {
            let mut c = '.';
            if (row, col) == (0, 0) {
                c = 's'
            }
            for i in (1..tails.len()).rev() {
                if (row, col) == tails[i] {
                    c = i.to_string().chars().next().unwrap();
                    break;
                }
            }
            if (row, col) == tails[0] {
                c = 'H';
            }
            print!("{}", c);
        }
        println!();
    }
    println!();
}

type Pos = (i32, i32);

fn follow(lead: &Pos, follower: &Pos) -> Option<Pos> {
    let d0 = (follower.0 - lead.0).abs();
    let d1 = (follower.1 - lead.1).abs();
    if d0 * d0 + d1 * d1 <= 2 {
        return None;
    }

    Some((
        follower.0 + (lead.0 - follower.0).signum() * d0.min(1),
        follower.1 + (lead.1 - follower.1).signum() * d1.min(1),
    ))
}

fn solve<const N: usize>(f: &str) -> usize {
    let mut pos = HashSet::new();
    let mut tails = [(0i32, 0i32); N];
    pos.insert(tails[N - 1]);
    fs::read_to_string(f)
        .expect("read failed")
        .split('\n')
        .filter(|l| l.len() > 0)
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
                tails[0].0 += d.0;
                tails[0].1 += d.1;

                let state = tails[0];
                tails[1..]
                    .iter_mut()
                    .scan(state, |state, item| {
                        if let Some(new_pos) = follow(state, item) {
                            *item = new_pos;
                            *state = *item;
                            Some(())
                        } else {
                            None
                        }
                    })
                    .count();
                pos.insert(tails[N - 1]);
            }
        })
        .count();
    pos.len()
}

fn main() {
    println!("part 1: {}", solve::<2>("input.txt"));
    println!("part 2: {}", solve::<10>("input.txt"));
}
