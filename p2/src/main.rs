use std::collections::HashMap;
use std::fs;

fn main() {
    fn score_q(q: &str) -> i32 {
        if q == "Y" {
            2
        } else if q == "X" {
            1
        } else if q == "Z" {
            3
        } else {
            0
        }
    }
    fn win(p: &str, q: &str) -> i32 {
        // A X rock,
        // B Y paper
        // C Z scissors
        let m = vec![
            ("A", "X", 3),
            ("A", "Y", 6),
            ("A", "Z", 0),

            ("B", "X", 0),
            ("B", "Y", 3),
            ("B", "Z", 6),

            ("C", "X", 6),
            ("C", "Y", 0),
            ("C", "Z", 3),
        ];
        for (p1, q1, s) in m {
            if p1 == p && q1 == q {
                return s;
            }
        }
        return -1000;
    }
    //let f = "test.txt";
    let f = "input.txt";
    let score = fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .map(|l| {
            let v = l.split(' ').collect::<Vec<&str>>();
            if v.len() < 2 {
                return 0;
            }
            let p = v[0];
            let q = v[1];
            println!("v='{:?}', {} + {}", v, score_q(q) , win(p, q));
            score_q(q) + win(p, q)
        })
        .sum::<i32>();
    println!("score: {}", score);
}
