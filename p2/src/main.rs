use std::collections::HashMap;
use std::fs;

fn main() {
    fn score_q(q: &str) -> i32 {
        if q == "B" {
            2
        } else if q == "A" {
            1
        } else if q == "C" {
            3
        } else {
            0
        }
    }
    fn play(p: &str, q:&str) -> String {
        // A rock,
        // B paper
        // C scissors
        // X lose Y draw Z win
        let m = vec![
            ("A", "X", "C"),
            ("A", "Y", "A"),
            ("A", "Z", "B"),
            ("B", "X", "A"),
            ("B", "Y", "B"),
            ("B", "Z", "C"),
            ("C", "X", "B"),
            ("C", "Y", "C"),
            ("C", "Z", "A"),
        ];
        for (p1, q1, s) in m {
            if p1 == p && q1 == q {
                return s.to_string();
            }
        }
        return "?".to_string();
    }
    fn win(p: &str, q: &str) -> i32 {
        // A rock,
        // B paper
        // C scissors
        let m = vec![
            ("A", "A", 3),
            ("A", "B", 6),
            ("A", "C", 0),
            ("B", "A", 0),
            ("B", "B", 3),
            ("B", "C", 6),
            ("C", "A", 6),
            ("C", "B", 0),
            ("C", "C", 3),
        ];
        for (p1, q1, s) in m {
            if p1 == p && q1 == q {
                return s;
            }
        }
        return -1000;
    }
    let f = "input.txt";
    //let f = "test.txt";
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
            let z = play(p, q);
            println!("v='{:?}', {} + {}", v, score_q(z.as_str()), win(p, z.as_str()));
            score_q(z.as_str()) + win(p, z.as_str())
        })
        .sum::<i32>();
    println!("score: {}", score);
}
