use std::fs;

fn read_input(f: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| l.as_bytes().to_vec())
        .collect()
}

fn solve_p1(f: &str) -> usize {
    let ts = read_input(f);
    let n = ts.len();
    let mut vs = vec![vec![0; n]; n];

    let mut max;
    for row in 0..n {
        max = 0;
        for col in 0..n {
            if ts[row][col] > max {
                vs[row][col] += 1;
            }
            max = max.max(ts[row][col]);
        }
    }
    for row in 0..n {
        max = 0;
        for col in (0..n).rev() {
            if ts[row][col] > max {
                vs[row][col] += 1;
            }
            max = max.max(ts[row][col]);
        }
    }
    for col in 0..n {
        max = 0;
        for row in 0..n {
            if ts[row][col] > max {
                vs[row][col] += 1;
            }
            max = max.max(ts[row][col]);
        }
    }
    for col in 0..n {
        max = 0;
        for row in (0..n).rev() {
            if ts[row][col] > max {
                vs[row][col] += 1;
            }
            max = max.max(ts[row][col]);
        }
    }
    vs.iter()
        .map(|row| row.iter().filter(|&&v| v > 0).count())
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(solve_p1("test.txt"), 21);
    }
    #[test]
    fn test_p2() {
        assert_eq!(solve_p2("test.txt"), 8);
    }

    #[test]
    fn test_blocks() {
        let mut b = Blocks::new();
        assert_eq!(b.insert(1), 0);
        assert_eq!(b.insert(1), 1);
        assert_eq!(b.insert(2), 2);
        assert_eq!(b.insert(4), 3);
        assert_eq!(b.insert(3), 1);
        assert_eq!(b.insert(1), 1);
        assert_eq!(b.insert(3), 2);

        let mut b = Blocks::new();
        assert_eq!(b.insert(9), 0);
        assert_eq!(b.insert(4), 1);
        assert_eq!(b.insert(5), 2);

        let mut b = Blocks::new();
        assert_eq!(b.insert(3), 0);
        assert_eq!(b.insert(5), 1);
    }
}

#[derive(Debug)]
struct Blocks {
    bs: Vec<(u8, usize)>,
    pos: usize,
}
impl Blocks {
    fn new() -> Self {
        Blocks {
            bs: vec![(255, 0)],
            pos: 0,
        }
    }
    fn insert(&mut self, height: u8) -> usize {
        //print!("insert {height}, was: {:?} become: ", self);
        let mut can_see = 0;
        let mut new_len = 1;
        for i in (0..self.bs.len()).rev() {
            if self.bs[i].0 > height {
                if can_see == 0 {
                    can_see = self.pos - self.bs[i].1;
                }
                new_len = i + 1;
                break;
            }
            if self.bs[i].0 == height {
                can_see = self.pos - self.bs[i].1;
            }
        }

        self.bs.truncate(new_len);
        self.bs.push((height, self.pos));
        self.pos += 1;
        //println!("{:?}", self);
        can_see
    }
}

fn solve_p2(f: &str) -> usize {
    let ts = read_input(f);
    let num = ts.len();
    let mut vs = vec![vec![[0; 4]; num]; num];

    for row in 0..num {
        let mut b = Blocks::new();
        for col in 0..num {
            vs[row][col][0] = b.insert(ts[row][col]);
        }
    }
    for row in 0..num {
        let mut b = Blocks::new();
        for col in (0..num).rev() {
            vs[row][col][1] = b.insert(ts[row][col]);
        }
    }
    for col in 0..num {
        let mut b = Blocks::new();
        for row in 0..num {
            vs[row][col][2] = b.insert(ts[row][col]);
        }
    }
    for col in 0..num {
        let mut b = Blocks::new();
        for row in (0..num).rev() {
            vs[row][col][3] = b.insert(ts[row][col]);
        }
    }
    /*
    for d in 0..4 {
        vs.iter()
            .map(|row| println!("{:?}", row.iter().map(|ds| ds[d]).collect::<Vec<usize>>()))
            .count();
        println!();
    }
     */

    vs.iter()
        .map(|row| {
            row.iter()
                .map(|ds| ds.iter().fold(1, |accum, &item| accum * item))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let f = "input.txt";
    println!("part1: {}", solve_p1(f));
    println!("part1: {}", solve_p2(f));
}
