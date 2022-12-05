use itertools::Itertools;
use std::fs;

fn print_tops(layout: &Vec<Vec<char>>) {
    layout
        .iter()
        .enumerate()
        .map(|(num, contents)| {
            if contents.len() > 0 {
                print!("{}", contents[contents.len() - 1]);
            }
        })
        .count();
        println!("");
}
fn print_layout(layout: &Vec<Vec<char>>) {
    layout
        .iter()
        .enumerate()
        .map(|(num, contents)| {
            println!("box {} contains {:?}", num, contents);
        })
        .count();
}
fn main() {
    //let f = "test.txt";
    let f = "input.txt";
    let (pre_layout, ops) = fs::read_to_string(f)
        .expect("read failed")
        .split('\n')
        .group_by(|l| l.len() == 0)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<&str>>())
        .filter(|v| v.len() > 1)
        .map(|g| g.iter().map(|s| s.to_string()).collect::<Vec<String>>())
        .collect_tuple()
        .unwrap();

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
    let mut layout = transpose(&pre_layout);
    print_layout(&layout);

    struct Op {
        num: usize,
        from: usize,
        to: usize,
    }
    ops.iter()
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
        .map(|op| {
            for _ in 0..op.num {
                let pop = layout[op.from].pop().unwrap();
                layout[op.to].push(pop);
            }
        })
        .count();
    println!("after");
    print_layout(&layout);
    print_tops(&layout);
}
