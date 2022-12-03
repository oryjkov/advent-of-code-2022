use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn char_to_prio(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn merge_lists(l: &[u32], r: &[u32]) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    let mut j = 0;
    while i < l.len() && j < r.len() {
        if l[i] == r[j] {
            let x = l[i];
            sum += l[i];
            while i < l.len() && l[i] == x {
                i += 1;
            }
            while j < r.len() && r[j] == x {
                j += 1;
            }
        } else if l[i] < r[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    sum
}
fn line_to_vecs(l: &str) -> (Vec<u32>, Vec<u32>) {
    let v: Vec<u32> = l.chars().map(char_to_prio).collect();
    let (mut l, mut r) = (v[0..v.len() / 2].to_vec(), v[v.len() / 2..v.len()].to_vec());
    l.sort();
    r.sort();
    (l, r)
}

fn solve1() {
    //let f = "test.txt";
    let f = "input.txt";
    let s = fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(line_to_vecs)
        .map(|(l, r)| merge_lists(&l, &r))
        .sum::<u32>();
    println!("{}", s);
}

fn solve2() {
    //let f = "test.txt";
    let f = "input.txt";
    let s = fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            let x: HashSet<u32> = HashSet::from_iter(l.chars().map(char_to_prio));
            x
        })
        .chunks(3)
        .into_iter()
        .map(|group| {
            let v = group.collect_vec();
            v[0].iter()
                .filter(|&x| v[1].contains(x) && v[2].contains(x))
                .copied()
                .collect::<Vec<u32>>()[0]
        })
        .sum::<u32>();
    println!("{}", s);
}

fn main() {
    solve2();
}
