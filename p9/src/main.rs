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
struct Position(i32, i32);
#[derive(Clone, Copy)]
struct Displacement(i32, i32);

impl Displacement {
    fn new(c: &str) -> Self {
        match c {
            "U" => Displacement(1, 0),
            "D" => Displacement(-1, 0),
            "L" => Displacement(0, -1),
            "R" => Displacement(0, 1),
            _ => Displacement(-1000, 0),
        }
    }
    fn norm2(&self) -> i32 {
        self.0 * self.0 + self.1 * self.1
    }
    fn unit(&self) -> Self {
        Displacement(self.0.signum(), self.1.signum())
    }
}

impl ops::Add<Displacement> for &Position {
    type Output = Position;
    fn add(self, dir: Displacement) -> Position {
        Position(self.0 + dir.0, self.1 + dir.1)
    }
}

impl ops::AddAssign<Displacement> for Position {
    fn add_assign(&mut self, dir: Displacement) {
        self.0 += dir.0;
        self.1 += dir.1;
    }
}

fn follow(lead: &Position, follower: &Position) -> Option<Position> {
    let dir = Displacement(lead.0 - follower.0, lead.1 - follower.1);
    if dir.norm2() <= 2 {
        return None;
    }
    Some(follower + dir.unit())
}

fn solve<const N: usize>(f: &str) -> usize {
    let mut pos = HashSet::new();
    let mut tails = [Position(0i32, 0i32); N];
    pos.insert(tails[N - 1]);
    fs::read_to_string(f)
        .expect("read failed")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            let input: Vec<&str> = l.split_whitespace().collect();
            (Displacement::new(input[0]), input[1].parse::<usize>().unwrap())
        })
        .map(|(d, n)| {
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
