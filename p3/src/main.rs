use std::fs;

fn main() {
    //let f = "test.txt";
    let f = "input.txt";
    let s = fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| {
            let v: Vec<u32> = l
                .chars()
                .map(|c| {
                    if c.is_lowercase() {
                        c as u32 - 'a' as u32 + 1
                    } else {
                        c as u32 - 'A' as u32 + 27
                    }
                })
                .collect();
            let (mut l, mut r) = (v[0..v.len() / 2].to_vec(), v[v.len() / 2..v.len()].to_vec());
            l.sort();
            r.sort();
            (l, r)
        })
        .map(|(l, r)| {
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
        })
        .sum::<u32>();
    println!("{}", s);
}
