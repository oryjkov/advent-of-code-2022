use scanf::*;
use std::collections::HashMap;
use std::fmt::format;
use std::{
    cell::{Cell, RefCell},
    fs,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_walk() {
        let m = read_in("test.txt");
        assert_eq!(find_max(&m, 3), 20);
        let m = read_in("test.txt");
        assert_eq!(find_max(&m, 4), 40);
        let m = read_in("test.txt");
        assert_eq!(find_max(&m, 5), 63);
        let m = read_in("test.txt");
        assert_eq!(find_max(&m, 6), 93);
        let m = read_in("test.txt");
        assert_eq!(find_max(&m, 7), 126);
        // another branch
        let m = read_in("test.txt");
        print_map(&m);
        assert_eq!(find_max(&m, 8), 162);
    }
    #[test]
    fn test_part1() {
        //assert_eq!(solve_part1("test.txt"), -1);
    }
    #[test]
    fn test_part2() {
        //assert_eq!(solve_part1("test.txt"), -1);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValveState {
    Open,
    Closed,
}
use ValveState::*;

#[derive(Debug, Clone)]
struct Room {
    name: String,
    rate: i32,
    state: RefCell<ValveState>,
    visits: RefCell<i32>,
    neibs: HashMap<String, i32>,
}

#[derive(Debug, Clone)]
struct PathElement {
    room: String,
    rate: i32,
}
type Map = HashMap<String, Room>;

fn condense(m: &mut Map) {
    loop {
        let mut cand = None;
        for node in m.iter() {
            if node.1.rate == 0 && node.1.neibs.len() == 2 {
                cand = Some(node.0.clone());
                break;
            }
        }
        if cand.is_none() {
            break;
        }
        // Remove the node and increase its neibs cost.
        let r = m.remove(&cand.unwrap()).unwrap();
        let mut iter = r.neibs.iter();
        let (left, &left_cost) = iter.next().unwrap();
        let (right, &right_cost) = iter.next().unwrap();
        //println!("removed: {}, left: {}, right: {}", r.name, left, right);
        //println!("was: {left}: {:?}, {right}: {:?}", m.get(left).unwrap().neibs, m.get(right).unwrap().neibs);
        {
            let left_neib = m.get_mut(left).unwrap();
            left_neib.neibs.remove(&r.name).unwrap();
            if left_neib.neibs.get(right).is_none() {
                left_neib.neibs.insert(right.clone(), left_cost+right_cost);
            }
        }

        {
            let right_neib = m.get_mut(right).unwrap();
            right_neib.neibs.remove(&r.name).unwrap();
            if right_neib.neibs.get(right).is_none() {
                right_neib.neibs.insert(left.clone(), left_cost+right_cost);
            }
        }
        //println!("now: {left}: {:?}, {right}: {:?}", m.get(left).unwrap().neibs, m.get(right).unwrap().neibs);
        //println!();
    }
}

fn print_map(m: &Map) {
    m.iter()
        .for_each(|(_, r)| println!("{}: r={}, {:?}, {:?}", r.name, r.rate, r.neibs, r.visits));
}

fn read_in(f: &str) -> Map {
    let mut m = HashMap::new();
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| {
            let mut v: String = String::new();
            let mut vs: String = String::new();
            let mut r: i32 = 0;
            if sscanf!(
                l,
                "Valve {} has flow rate={}; tunnels lead to valves {string}",
                v,
                r,
                vs
            )
            .is_err()
            {
                sscanf!(
                    l,
                    "Valve {} has flow rate={}; tunnel leads to valve {string}",
                    v,
                    r,
                    vs
                )
                .unwrap();
            }
            (
                v,
                r,
                vs.split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .for_each(|(v, r, vs)| {
            let mut hm = HashMap::new();
            vs.iter().for_each(|v| {
                hm.insert(v.clone(), 1);
            });
            m.insert(
                v.clone(),
                Room {
                    name: v,
                    rate: r,
                    state: RefCell::new(Closed),
                    visits: RefCell::new(vs.len() as i32),
                    neibs: hm,
                },
            );
        });

    m
}
fn solve_part1(f: &str) -> i32 {
    let mut m = read_in(f);
    condense(&mut m);
    print_map(&m);
    find_max(&m, 30)
}

fn print_path(p: &Vec<PathElement>) {
    p.iter().for_each(|p| print!("{},", p.room));
    println!();
}
fn path_cost(p: &Vec<PathElement>, len: usize) -> i32 {
    let mut acc = 0;
    for i in 0..p.len() {
        acc += p[i].rate * (len as i32 - i as i32);
    }
    acc
}

fn find_max(m: &Map, len: usize) -> i32 {
    let max = Cell::new(0);
    let remaining = m.iter().filter(|(_, r)| r.rate > 0).count();
    walk(&m, len as i32, "AA", &mut vec![], remaining, &mut |p| {
        let m = max.get();
        let c = path_cost(p, len - 1);
        if c > m {
            max.set(c);
            print!("max({}): {}, ", len, c);
            print_path(p);
        }
        //print!("cost: {}, ", c); print_path(p);
    });
    max.get()
}

fn walk<F>(m: &Map, budget: i32, from: &str, p: &mut Vec<PathElement>, remaining: usize, f: &mut F)
where
    F: FnMut(&Vec<PathElement>),
{
    let node = m.get(from).unwrap();
    if budget <= 0 || remaining <= 0 {
        f(p);
        return;
    }
    if *node.visits.borrow() < 0 {
        f(p);
        return;
    }
    if node.rate > 0 && *node.state.borrow() == Closed {
        // Try to open first.
        p.push(PathElement {
            room: from.to_string()+"_O",
            rate: node.rate,
        });
        *node.state.borrow_mut() = Open;

        walk(m, budget - 1, from, p, remaining - 1, f);
        *node.state.borrow_mut() = Closed;
        p.pop();
    }
    *node.visits.borrow_mut() -= 1;
    for (neib_name, &cost) in node.neibs.iter() {
        for i in 0..cost {
            p.push(PathElement {
                room: neib_name.clone()+format!("_{}", i).as_str(),
                rate: 0,
            });
        }
        walk(m, budget - cost, neib_name, p, remaining, f);
        for _ in 0..cost {
            p.pop();
        }
    }
    *node.visits.borrow_mut() += 1;
}

fn solve_part2(f: &str) -> i32 {
    /*
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .count();
     */
    -1
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
