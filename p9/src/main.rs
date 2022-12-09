use std::{collections::HashSet, fs};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve::<2>("test.txt"), 13);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve::<10>("test2.txt"), 36);
    }
}

fn visualize(tails: &[(i32, i32)]) {
    let (width, height) = (5, 5);
    for row in (0..height).rev() {
        for col in 0..width {
            let mut c = '.';
            if (row, col) == (0, 0) {
                c = 's'
            }
            for i in (1..tails.len()).rev() {
                if (row, col) == tails[i] {
                    c = i.to_string().chars().next().unwrap();
                    break;
                }
            }
            if (row, col) == tails[0] {
                c = 'H';
            }
            print!("{}", c);
        }
        println!();
    }
    println!();
}

type Pos = (i32, i32);
type Dir = (i32, i32);

fn follow(lead: &Pos, follower: &Pos) -> Option<(Pos, Dir)> {
    let d0 = (follower.0 - lead.0).abs();
    let d1 = (follower.1 - lead.1).abs();
    if d0 * d0 + d1 * d1 <= 2 {
        return None;
    }

    let new_pos = (
        follower.0 + (lead.0 - follower.0).signum() * d0.min(1),
        follower.1 + (lead.1 - follower.1).signum() * d1.min(1),
    );
    let new_dir = (new_pos.0 - follower.0, new_pos.1 - follower.1);
    Some((new_pos, new_dir))
}

fn apply(direction: &Dir, tails: &mut [(i32, i32)]) {
    let mut dir = direction.clone();
    let mut ts = tails;
    loop {
        if let Some((head, new_ts)) = ts.split_first_mut() {
            ts = new_ts;
            head.0 += dir.0;
            head.1 += dir.1;

            if ts.len() == 0 {
                break;
            }
            if let Some((_, new_dir)) = follow(head, &ts[0]) {
                dir = new_dir;
            } else {
                break;
            };
        } else {
            break;
        }
    }
}

fn solve<const N: usize>(f: &str) -> usize {
    let mut pos = HashSet::new();
    //let mut h = (0i32, 0i32);
    let mut tails = [(0i32, 0i32); N];
    pos.insert(tails[N - 1]);
    //visualize(&tails);
    fs::read_to_string(f)
        .expect("read failed")
        .split('\n')
        .filter(|l| l.len() > 0)
        //.take(1)
        .map(|l| {
            let input: Vec<&str> = l.split_whitespace().collect();
            let d = match input[0] {
                "U" => (1, 0),
                "D" => (-1, 0),
                "L" => (0, -1),
                "R" => (0, 1),
                _ => (-1000, 0),
            };
            let n = input[1].parse::<usize>().unwrap();
            for _ in 0..n {
                apply(&d, &mut tails);
                /*
                println!("move");
                tails
                    .iter_mut()
                    .scan((&mut h, d), |accum, item| {
                        let lead = &mut accum.0;
                        let direction = &accum.1;
                        //println!("pre: lead: {:?} dir: {:?} follow: {:?}", lead, direction, item);
                        if let Some((_, new_dir)) = follow(lead, direction, item) {
                            //println!("after: {:?} {:?}, new_lead: {:?}", lead, new_dir, item);
                            //item.0 = new_pos.0;
                            //item.1 = new_pos.1;
                            accum.0 = item;
                            accum.1 = new_dir;
                            Some(1) //(item, new_dir))
                        } else {
                            //println!("after: {:?}", lead);
                            None
                        }
                    })
                    .count();
                    */
                //visualize(&tails);
                pos.insert(tails[N - 1]);
            }
        })
        .count();
    pos.len()
}

fn main() {
    println!("part 1: {}", solve::<2>("input.txt"));
    println!("part 2: {}", solve::<10>("input.txt"));
}
