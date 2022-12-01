use itertools::Itertools;
use std::fs;

fn main() {
    let top_n = 1;
    let f = "input.txt";
    let s = fs::read_to_string(f)
        .expect("read fail")
        .split("\n")
        .map(|l| {
            if l.len() == 0 {
                0
            } else {
                l.parse::<i32>().unwrap()
            }
        })
        .group_by(|&e| e == 0)
        .into_iter()
        .filter(|(k, _)| !*k)
        .map(|(k, v)| v.sum::<i32>())
        .sorted()
        .rev()
        .take(top_n)
        .sum::<i32>();
    println!("{:?}", s);
}
