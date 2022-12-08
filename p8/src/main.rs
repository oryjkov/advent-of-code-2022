use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(solve_p1("test.txt"), 21);
    }
}

fn solve_p1(f: &str) -> usize {
    let ts = fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let n = ts.len();
    let mut vs = vec![vec![0; n]; n];

    let mut max = 0;
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

fn main() {
    let f = "input.txt";
    println!("part1: {}", solve_p1(f));
}
