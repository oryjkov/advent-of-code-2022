use std::fs;
struct Monkey {
    items: Vec<i64>,
    operation: Op,
    next_monkey: Box<dyn Fn(i64) -> usize>,
    count: usize,
}

impl Monkey {
    fn run(&mut self) -> Vec<(i64, usize)> {
        let mut count = 0;
        let rv = self
            .items
            .iter()
            .map(|worry_level| {
                //let new_level = self.operation.eval(*worry_level) / 3;
                let new_level = self.operation.eval(*worry_level) % 9699690;
                //let new_level = self.operation.eval(*worry_level) % 96577;
                println!("{} {new_level}",*worry_level);
                let next_monkey = (*self.next_monkey)(new_level);
                count += 1;
                (new_level, next_monkey)
            })
            .collect();
        self.items.clear();
        self.count += count;

        rv
    }
}
fn apply_throws(throws: Vec<(i64, usize)>, monkeys: &mut Vec<Monkey>) {
    throws
        .iter()
        .map(|(worry_level, monkey_index)| {
            monkeys[*monkey_index].items.push(*worry_level);
        })
        .count();
}

enum Val {
    Const(i64),
    Old,
}
impl Val {
    fn new(s: &str) -> Val {
        if s == "old" {
            Val::Old
        } else {
            Val::Const(s.parse().unwrap())
        }
    }
    fn eval(&self, x: i64) -> i64 {
        match self {
            Val::Old => x,
            Val::Const(i) => *i,
        }
    }
}

struct Op {
    left: Val,
    right: Val,
    op: Box<dyn Fn(i64, i64) -> i64>,
}
impl Op {
    fn new(s: &str) -> Op {
        let ps: Vec<&str> = s.split_whitespace().collect();
        let left = Val::new(ps[0]);
        let right = Val::new(ps[2]);
        let op = Box::new(match ps[1] {
            "+" => |a, b| a + b,
            "-" => |a, b| a - b,
            "*" => |a, b| {println!("a={a} b={b}");a * b},
            "/" => |a, b| a / b,
            _ => |_, _| 0,
        });
        Op { left, right, op }
    }
    fn eval(&self, x: i64) -> i64 {
        (*self.op)(self.left.eval(x), self.right.eval(x))
    }
}

fn parse_monkey(t: &str) -> Monkey {
    let s: Vec<&str> = t.split('\n').collect();
    let items = s[1].split(": ").collect::<Vec<&str>>()[1]
        .split(", ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    //println!("items: {:?}", items);
    let operation = Op::new(s[2].split("new = ").collect::<Vec<&str>>()[1]);
    //println!("op: {:?}", operation);
    let div = s[3].split("divisible by ").collect::<Vec<&str>>()[1]
        .parse::<i64>()
        .unwrap();
    //println!("dif: {:?}", div);
    let t = s[4].split("to monkey ").collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    let f = s[5].split("to monkey ").collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    //println!("t,f: {t} {f}");
    let next_monkey = Box::new(move |x| if x % div == 0 { t } else { f });

    Monkey {
        items,
        operation: operation,
        next_monkey,
        count: 0,
    }
}

fn solve_p1(f: &str) -> i64 {
    let mut monkeys: Vec<Monkey> = fs::read_to_string(f)
        .unwrap()
        .split("\n\n")
        .filter(|l| l.len() > 0)
        .map(parse_monkey)
        .collect();
    (0..10_000)
        .map(|_| {
            let len = monkeys.len();
            for i in 0..len {
                let throws = monkeys[i].run();
                //println!("{i}: {:?}", throws);
                apply_throws(throws, &mut monkeys);
            }
            /*
            monkeys
                .iter()
                .enumerate()
                .map(|(i, monkey)| println!("monkey {i}, items: {:?}", monkey.items))
                .count();
             */
        })
        .count();
    let mut v: Vec<usize> = monkeys.iter().map(|m| m.count).collect();
    v.sort();
    (v[v.len()-1]*v[v.len()-2]) as i64
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(10605, solve_p1("test.txt"))
    }
}
fn main() {
    println!("part 1: {}", solve_p1("input.txt"));
}
