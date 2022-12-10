use std::fs;
enum Instr {
    Nop,
    Add(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(13140, solve_p1("test.txt"));
    }
}

fn make_states(f: &str) -> Vec<i32> {
    fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|s| {
            let i: Vec<&str> = s.split_whitespace().collect();
            if i.len() == 1 {
                Instr::Nop
            } else {
                Instr::Add(i[1].parse().unwrap())
            }
        })
        .map(|i| match i {
            Instr::Nop => vec![0],
            Instr::Add(x) => vec![0, x],
        })
        .into_iter()
        .reduce(|mut a, mut b| {
            a.append(&mut b);
            a
        })
        .unwrap()
}

fn solve_p1(f: &str) -> i32 {
    let xs = make_states(f);
    let mut x = 1;
    let mut sum = 0;
    for (i, cycle) in xs.iter().enumerate() {
        let cycle_num = i + 1;
        if cycle_num % 40 == 20 {
            sum += cycle_num as i32 * x;
            println!("{cycle_num}: {}", cycle_num as i32 * x);
        }
        x += cycle;
    }
    sum
}

fn solve_p2(f: &str) {
    let xs = make_states(f);
    let mut x = 1;
    for (i, cycle) in xs.iter().enumerate() {
        let cycle_num = i + 1;
        let col = i % 40;
        let d = if (col as i32 - x).abs() <= 1 {
            "#"
        } else {
            "."
        };
        print!("{d}");
        if col == 39 {
            println!()
        }
        x += cycle;
    }
}

fn main() {
    let f = "input.txt";
    let sum = solve_p1(f);
    println!("part1: {sum}");
    println!("part2:");
    solve_p2(f);
}
