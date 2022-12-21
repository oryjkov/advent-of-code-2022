use std::{fs, collections::HashMap};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 152);
        assert_eq!(solve_part1("input.txt"), 78342931359552);
    }
    #[test]
    fn test_part2() {
    }
}

#[derive(Clone)]
struct Operation {
    op: char,
    left: String,
    right: String,
}

#[derive(Clone)]
enum Monkey {
    Val(i64),
    Op(Operation),
}
use Monkey::*;
type Monkeys = HashMap<String, Monkey>;

fn eval(root: &str, monkeys: &mut Monkeys) -> i64 {
    let r = monkeys.get(root).unwrap().clone();
    let v = match r {
        Val(v) => v,
        Op(op) => {
            let l = eval(&op.left, monkeys);
            let r = eval(&op.right, monkeys);
            match op.op {
                '+' => l+r,
                '-' => l-r,
                '/' => l/r,
                '*' => l*r,
                _ => panic!("wrong op char"),
            }

        },
    };
    *monkeys.get_mut(root).unwrap() = Val(v);
    v
}

fn solve_part1(f: &str) -> i64 {
    let mut monkeys = HashMap::new();
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len()>0)
        .map(|l| {
            let mut i = l.split(": ");
            let name = i.next().unwrap().to_string();
            let op = i.next().unwrap();
            let monkey = if let Some(v) = op.parse::<i64>().ok() {
                Val(v)
            } else {
                let mut j = op.split_whitespace();
                let l = j.next().unwrap().to_string();
                let char_op = j.next().unwrap().chars().next().unwrap();
                let r = j.next().unwrap().to_string();
                Op(Operation { op: char_op, left: l, right: r })
            };
            (name, monkey)
        }).for_each(|(name, monkey)| {
            monkeys.insert(name, monkey);
        });
    eval("root", &mut monkeys)
}

fn solve_part2(f: &str) -> i32 {
    /*
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .count();
     */
    -1
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
