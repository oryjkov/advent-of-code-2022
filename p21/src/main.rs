use std::{collections::HashMap, fs};

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
        assert_eq!(solve_part2("test.txt"), 301);
        assert_eq!(solve_part2("input.txt"), 3296135418820);
    }
}

#[derive(Clone, Debug)]
struct Operation {
    op: char,
    left: String,
    right: String,
}

#[derive(Clone, Debug)]
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
            find_top(op.op, l, r)
        }
    };
    *monkeys.get_mut(root).unwrap() = Val(v);
    v
}

fn find_top(op: char, l: i64, r: i64) -> i64 {
    match op {
        '+' => l + r,
        '-' => l - r,
        '/' => l / r,
        '*' => l * r,
        _ => panic!("wrong op char"),
    }
}

fn find_left(op: char, top: i64, right: i64) -> i64 {
    match op {
        '+' => top - right,
        '-' => top + right,
        '/' => top * right,
        '*' => top / right,
        _ => panic!("wrong op char"),
    }
}

fn find_right(op: char, top: i64, left: i64) -> i64 {
    match op {
        '+' => top - left,
        '-' => left - top,
        '/' => left / top,
        '*' => top / left,
        _ => panic!("wrong op char"),
    }
}

fn try_eval(root: &str, monkeys: &mut Monkeys) -> Option<i64> {
    if root == "humn" {
        return None;
    }
    let r = monkeys.get(root).unwrap().clone();
    let result = match r {
        Val(v) => Some(v),
        Op(op) => {
            let l = try_eval(&op.left, monkeys);
            let r = try_eval(&op.right, monkeys);
            match (l, r) {
                (None, None) => panic!("abc"),
                (Some(lval), Some(rval)) => Some(find_top(op.op, lval, rval)),
                _ => None,
            }
        }
    };
    if let Some(v) = result {
        *monkeys.get_mut(root).unwrap() = Val(v);
    }
    result
}
fn walk_down(monkeys: &mut Monkeys) -> i64 {
    let r = monkeys.get("root").unwrap().clone();
    let (mut root_name, mut x) = match r {
        Op(op) => {
            let left = monkeys.get(&op.left).clone().unwrap();
            let right = monkeys.get(&op.right).clone().unwrap();
            match (left, right) {
                (Op(_), Val(rval)) => (op.left.clone(), *rval),
                (Val(lval), Op(_)) => (op.right.clone(), *lval),
                _ => panic!(""),
            }
        }
        _ => panic!("root is not an Op node, but {:?}!", r),
    };
    while root_name != "humn".to_string() {
        let r = monkeys.get(&root_name).unwrap().clone();
        //println!("following monkey '{}' = {:?}", root_name, r);
        (root_name, x) = match r {
            Op(op) => {
                let left = monkeys.get(&op.left).clone().unwrap();
                let right = monkeys.get(&op.right).clone().unwrap();
                match (left, right) {
                    (Op(_), Val(rval)) => (op.left.clone(), find_left(op.op, x, *rval)),
                    (Val(lval), Op(_)) => (op.right.clone(), find_right(op.op, x, *lval)),
                    _ => panic!(
                        "node {:?} is not an op/val, but: {:?} {:?}",
                        op, left, right
                    ),
                }
            }
            _ => panic!("node {} is not an Op node but {:?}", root_name, r),
        };
    }
    x
}

fn parse_input(f: &str) -> Monkeys {
    let mut monkeys = HashMap::new();
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
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
                Op(Operation {
                    op: char_op,
                    left: l,
                    right: r,
                })
            };
            (name, monkey)
        })
        .for_each(|(name, monkey)| {
            monkeys.insert(name, monkey);
        });
    monkeys
}

fn solve_part1(f: &str) -> i64 {
    let mut monkeys = parse_input(f);
    eval("root", &mut monkeys)
}

fn solve_part2(f: &str) -> i64 {
    let mut monkeys = parse_input(f);
    monkeys.insert(
        "humn".to_string(),
        Op(Operation {
            op: '!',
            left: "cant see".to_string(),
            right: "cant hear".to_string(),
        })
    );
    try_eval("root", &mut monkeys);
    walk_down(&mut monkeys)
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("test.txt"));
}
