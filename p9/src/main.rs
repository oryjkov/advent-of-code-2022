use std::{collections::HashSet, fs, ops};

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

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Pos(i32, i32);
#[derive(Clone, Copy)]
struct Dir(i32, i32);

impl Dir {
    fn new(c: &str) -> Self {
        match c {
            "U" => Dir(1, 0),
            "D" => Dir(-1, 0),
            "L" => Dir(0, -1),
            "R" => Dir(0, 1),
            _ => Dir(-1000, 0),
        }
    }
    fn norm2(&self) -> i32 {
        self.0 * self.0 + self.1 * self.1
    }
    fn capped(&self) -> Self {
        Dir(
            self.0.signum(),
            self.1.signum(),
        )
    }
}

impl ops::Add<Dir> for &Pos {
    type Output = Pos;
    fn add(self, dir: Dir) -> Pos {
        Pos(self.0 + dir.0, self.1 + dir.1)
    }
}

impl ops::AddAssign<Dir> for Pos {
    fn add_assign(&mut self, dir: Dir) {
        self.0 += dir.0;
        self.1 += dir.1;
    }
}

fn follow(lead: &Pos, follower: &Pos) -> Option<Pos> {
    let dir = Dir(lead.0 - follower.0, lead.1 - follower.1);
    if dir.norm2() <= 2 {
        return None;
    }
    Some(follower+dir.capped())
}

fn solve<const N: usize>(f: &str) -> usize {
    let mut pos = HashSet::new();
    let mut tails = [Pos(0i32, 0i32); N];
    pos.insert(tails[N - 1]);
    fs::read_to_string(f)
        .expect("read failed")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            let input: Vec<&str> = l.split_whitespace().collect();
            let d = Dir::new(input[0]);
            let n = input[1].parse::<usize>().unwrap();
            for _ in 0..n {
                tails[0] += d;

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
