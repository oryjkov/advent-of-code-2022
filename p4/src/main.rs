use std::fs;

#[derive(Copy, Clone, Debug)]
struct Interval {
    a: usize,
    b: usize,
}
impl Interval {
    fn contains(&self, other: &Interval) -> bool {
        self.a <= other.a && self.b >= other.b
    }
    fn contains2(&self, other: &Interval) -> bool {
        self.contains(other) || other.contains(self)
    }
}

fn main() {
    let f = "input.txt";
    let s = fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            let s: Vec<Interval> = l.split(',').map(|p| {
                let v: Vec<usize> = p.split('-').map(|n| n.parse::<usize>().unwrap()).collect();
                Interval { a: v[0], b: v[1] }
            }).collect();
            if s[0].contains2(&s[1]) {1}else{0}
        }).sum::<usize>();
    println!("{s}");
}
