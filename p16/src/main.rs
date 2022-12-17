use scanf::*;
use std::collections::{HashMap, HashSet};
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
        assert_eq!(solve_part1("test.txt"), 1651);
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
                left_neib
                    .neibs
                    .insert(right.clone(), left_cost + right_cost);
            }
        }

        {
            let right_neib = m.get_mut(right).unwrap();
            right_neib.neibs.remove(&r.name).unwrap();
            if right_neib.neibs.get(right).is_none() {
                right_neib
                    .neibs
                    .insert(left.clone(), left_cost + right_cost);
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
    print_map(&m);
    //condense(&mut m);
    //find_max(&m, 30)

    let mut shortest_paths = HashMap::new();
    for (node, _) in m.iter() {
        let s = shortest_from(&m, node);
        for (n, dist) in s.iter() {
            shortest_paths.insert((node.clone(), n.clone()), *dist);
            shortest_paths.insert((n.clone(), node.clone()), *dist);
        }
    }
    find_max_fan(&m, 30, &shortest_paths)
}

fn solve_part2(f: &str) -> i32 {
    let mut m = read_in(f);
    let mut shortest_paths = HashMap::new();
    for (node, _) in m.iter() {
        let s = shortest_from(&m, node);
        for (n, dist) in s.iter() {
            shortest_paths.insert((node.clone(), n.clone()), *dist);
            shortest_paths.insert((n.clone(), node.clone()), *dist);
        }
    }
    find_all(&m, 26, &shortest_paths)
}

// A hacky way to solve part 2. It generates all the possible solutions with a single actor
// for times ranging from 1..26 minutes. Then for each of those solution it solves for the second
// actor trying to visit the remaining rooms in 26 minutes. Runs in around 6.5 minutes.
fn find_all(m: &Map, len: usize, shortest_paths: &HashMap<(String, String), i32>) -> i32 {
    let from = "AA";
    let mut max = 0;

    for budget in 1..26 {
        let mut paths = vec![];

        let mut visited = HashSet::new();
        visited.insert(from.to_string());
        walk_fan(
            &m,
            budget,
            from,
            &mut vec![],
            &mut visited,
            shortest_paths,
            &mut |p| {
                paths.push(p.clone());
            },
        );
        println!("budget: {} found {} paths", budget, paths.len());
        for p1 in &paths {
            let mut visited = HashSet::new();
            for el in p1 {
                if el.rate > 0 {
                    visited.insert(el.room[0..2].to_string());
                }
            }
            let c1 = path_cost(p1, len - 1);

            walk_fan(
                &m,
                len as i32,
                from,
                &mut vec![],
                &mut visited,
                shortest_paths,
                &mut |p2| {
                    let c2 = path_cost(p2, len - 1);
                    if c1 + c2 > max {
                        max = c1 + c2;
                        println!("max({}):", max);
                        print_path(p1);
                        print_path(p2);
                        println!();
                    }
                },
            );
        }
    }

    -1
}

// This version picks the next valve to open and goes straight to it.
// Compared to walk() each decision has more options (at least while there are many closed
// valves), but the recursion depth should be shorter.
fn walk_fan<F>(
    m: &Map,
    budget: i32,
    from: &str,
    path: &mut Vec<PathElement>,
    visited: &mut HashSet<String>,
    shortest_paths: &HashMap<(String, String), i32>,
    f: &mut F,
) where
    F: FnMut(&Vec<PathElement>),
{
    let mut candidates = vec![];
    for (node, room) in m.iter() {
        if room.rate <= 0 {
            continue;
        }
        if visited.contains(node) {
            continue;
        }
        let shortest_path = *shortest_paths
            .get(&(from.to_string(), node.clone()))
            .unwrap();
        if shortest_path >= budget {
            continue;
        }
        candidates.push((node.clone(), room.rate, shortest_path));
    }
    if candidates.len() == 0 {
        // all valves are open
        f(path);
        return;
    }
    candidates.sort_by(|c1, c2| c2.1.cmp(&c1.1));
    for (candidate, rate, cost) in candidates {
        visited.insert(candidate.clone());
        // need to get there, then open the valve.
        for i in 0..cost {
            path.push(PathElement {
                room: candidate.clone() + format!("_{}", i).as_str(),
                rate: 0,
            });
        }
        path.push(PathElement {
            room: candidate.clone() + "_O",
            rate: rate,
        });
        walk_fan(
            m,
            budget - cost - 1,
            &candidate,
            path,
            visited,
            shortest_paths,
            f,
        );
        for _ in 0..cost + 1 {
            path.pop();
        }
        visited.remove(&candidate);
    }
}

fn print_path(p: &Vec<PathElement>) {
    p.iter().for_each(|p| print!("{},", p.room));
    println!();
}
fn path_cost(p: &Vec<PathElement>, len: usize) -> i32 {
    let mut acc = 0;
    for i in 0..p.len().min(len) {
        acc += p[i].rate * (len as i32 - i as i32);
    }
    acc
}

fn find_max_fan(m: &Map, len: usize, shortest_paths: &HashMap<(String, String), i32>) -> i32 {
    let from = "AA";
    let max = Cell::new(0);
    let checked = Cell::new(0);
    let mut visited = HashSet::new();
    visited.insert(from.to_string());
    walk_fan(
        &m,
        len as i32,
        from,
        &mut vec![],
        &mut visited,
        shortest_paths,
        &mut |p| {
            let m = max.get();
            let ch = checked.get();
            let c = path_cost(p, len - 1);
            if c > m {
                max.set(c);
                print!("checked: {ch}, max({}): {}, ", len, c);
                print_path(p);
            }
            if ch % 1_000_000 == 0 {
                println!("checked: {}", ch);
            }

            checked.set(ch + 1);
        },
    );
    max.get()
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

fn shortest_from(m: &Map, from: &str) -> HashMap<String, i32> {
    let mut unvisited = HashSet::new();
    for x in m.iter() {
        unvisited.insert(x.0.clone());
    }
    unvisited.remove(from);

    let mut distances = HashMap::new();
    distances.insert(from.to_string(), 0);
    let mut node = m.get(from).unwrap();
    loop {
        let &d1 = distances.get(&node.name).unwrap();
        for (neib, &d2) in &node.neibs {
            if !unvisited.contains(neib) {
                continue;
            }
            let dist_via = d1 + d2;
            if let Some(f) = distances.get_mut(neib) {
                if *f > dist_via {
                    *f = dist_via;
                }
            } else {
                distances.insert(neib.clone(), dist_via);
            }
        }
        let mut min_dist = 1000;
        let mut min_node = "".to_string();
        for n in unvisited.iter() {
            if let Some(c) = distances.get(n) {
                if min_dist > *c {
                    min_node = n.clone();
                    min_dist = *c;
                }
            }
        }
        if min_dist == 1000 {
            break;
        }
        unvisited.remove(&node.name);
        node = m.get(&min_node).unwrap();
    }
    distances
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
    let found = false;
    // See if we've been here already without opening any valves.
    for n in p.iter().rev() {
        if n.rate > 0 {
            break;
        }
        if n.room.starts_with(from) {
            break;
        }
    }
    if found {
        f(p);
        return;
    }
    if node.rate > 0 && *node.state.borrow() == Closed {
        // Try to open first.
        p.push(PathElement {
            room: from.to_string() + "_O",
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
                room: neib_name.clone() + format!("_{}", i).as_str(),
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

fn main() {
    //println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("test.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
