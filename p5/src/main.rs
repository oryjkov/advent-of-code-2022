use itertools::Itertools;
use std::fs;

type Layout = Vec<Vec<char>>;

fn tops(layout: &Layout) -> String {
    let mut s = "".to_string();
    layout
        .iter()
        .enumerate()
        .map(|(num, contents)| {
            if contents.len() > 0 {
                s.push(contents[contents.len() - 1]);
            }
        })
        .count();
    s
}
fn print_layout(layout: &Layout) {
    layout
        .iter()
        .enumerate()
        .map(|(num, contents)| {
            println!("box {} contains {:?}", num, contents);
        })
        .count();
}

// Converts the layout as given in the input into a list of boxs. Idea is to reverse then
// transpose the input lines and read out contents in order.
fn transpose(pre_layout: &[String]) -> Vec<Vec<char>> {
    let a = pre_layout
        .iter()
        .rev()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    //println!("{:?}", a);
    let mut rv: Vec<Vec<char>> = vec![];
    let column = &a[0];
    for (i, b_num) in column.iter().enumerate() {
        if *b_num == ' ' {
            continue;
        }
        let mut line: Vec<char> = vec![];
        for j in 1..a.len() {
            if a[j][i] == ' ' {
                continue;
            }
            line.push(a[j][i]);
        }
        rv.push(line);
    }
    rv
}
struct Op {
    num: usize,
    from: usize,
    to: usize,
}

fn do_op(op: &Op, layout: &mut Layout) {
    for _ in 0..op.num {
        let pop = layout[op.from].pop().unwrap();
        layout[op.to].push(pop);
    }
}

fn do_op_buf(op: &Op, layout: &mut Layout) {
    let l = layout[op.from].len();
    let pop = layout[op.from].split_off(l - op.num);
    layout[op.to].extend(pop);
}

fn parse_input(f: &str) -> (Layout, Vec<Op>) {
    let (pre_layout, pre_ops) = fs::read_to_string(f)
        .expect("read failed")
        .split('\n')
        .group_by(|l| l.len() == 0)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<&str>>())
        .filter(|v| v.len() > 1)
        .map(|g| g.iter().map(|s| s.to_string()).collect::<Vec<String>>())
        .collect_tuple()
        .unwrap();

    let mut layout = transpose(&pre_layout);
    let ops = pre_ops
        .iter()
        .map(|op_str| {
            let (num, from, to) = op_str
                .split(' ')
                .map(|word| word.to_string().parse::<usize>())
                .filter(|res| res.is_ok())
                .map(|res| res.unwrap())
                .collect_tuple()
                .unwrap();
            Op {
                num,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect::<Vec<Op>>();

    (layout, ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_p1() {
        assert_eq!(solve_p1("test.txt"), "CMZ");
    }
    #[test]
    fn test_solve_p2() {
        assert_eq!(solve_p2("test.txt"), "MCD");
    }
}
fn solve_p1(f: &str) -> String {
    let (mut layout, ops) = parse_input(f);
    ops.iter().map(|op| do_op(&op, &mut layout)).count();
    tops(&layout)
}
fn solve_p2(f: &str) -> String {
    let (mut layout, ops) = parse_input(f);
    ops.iter().map(|op| do_op_buf(&op, &mut layout)).count();
    tops(&layout)
}
fn main() {
    let f = "input.txt";
    println!("{}", solve_p2(f));
}
