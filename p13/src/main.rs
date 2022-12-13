use std::{cmp::Ordering, fs, str::from_utf8};

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListOrInt {
    Int(i32),
    List(Vec<ListOrInt>),
}
use ListOrInt::*;
//struct Packet { //list: }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_consume_int() {
        assert_eq!(consume_int(b"test.txt"), None);
        assert_eq!(consume_int(b",28"), None);
        assert_eq!(consume_int(b"28"), Some((28, 2)));
        assert_eq!(consume_int(b"28,"), Some((28, 2)));
        assert_eq!(consume_int(&b"28,10"[3..]), Some((10, 2)));
    }
    #[test]
    fn test_consume_list() {
        assert_eq!(consume_list(b"1"), None);
        assert_eq!(consume_list(b"[1]").unwrap().0, List(vec![Int(1)]));
        assert_eq!(consume_list(b"[1,]").unwrap().0, List(vec![Int(1)]));
        assert_eq!(
            consume_list(b"[1,2]").unwrap().0,
            List(vec![Int(1), Int(2)])
        );
        assert_eq!(
            consume_list(b"[1,[2],2]").unwrap().0,
            List(vec![Int(1), List(vec!(Int(2))), Int(2)])
        );
    }
    #[test]
    fn test_compare() {
        let l1 = consume_list(b"[1,1,3]").unwrap().0;
        let l2 = consume_list(b"[1,1,5]").unwrap().0;
        assert_eq!(compare(&l1, &l2), Some(true));
        let l1 = consume_list(b"[1,1,1]").unwrap().0;
        let l2 = consume_list(b"[1,1,1]").unwrap().0;
        assert_eq!(compare(&l1, &l2), None);
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 13);
        assert_eq!(solve_part1("input.txt"), 5938);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("test.txt"), 140);
    }
}

fn consume_int(s: &[u8]) -> Option<(i32, usize)> {
    let mut i = 0;
    while i < s.len() && s[i] >= b'0' && s[i] <= b'9' {
        i += 1;
    }
    if i == 0 {
        return None;
    }
    Some((from_utf8(&s[0..i]).unwrap().parse::<i32>().unwrap(), i))
}

fn consume_list(s: &[u8]) -> Option<(ListOrInt, usize)> {
    let mut rv = vec![];
    let mut consumed = 0;
    if s[consumed] != b'[' {
        // println!( "expected '[', got {}, input: {}", s[consumed] as char, from_utf8(s).unwrap());
        return None;
    }
    //println!("called on {}", from_utf8(s).unwrap());
    consumed += 1;
    loop {
        if consumed < s.len() && s[consumed] == b',' {
            consumed += 1;
        }
        let cons = {
            if let Some((cons_int, num_cons)) = consume_int(&s[consumed..]) {
                //print!("consumed {:?}", cons_int);
                consumed += num_cons;
                Some(ListOrInt::Int(cons_int))
            } else if let Some((cons_list, num_cons)) = consume_list(&s[consumed..]) {
                //print!("consumed {:?}", cons_list);
                consumed += num_cons;
                Some(cons_list)
            } else {
                None
            }
        };
        if cons.is_none() {
            //println!(".. found nothing, break");
            break;
        }
        rv.push(cons.unwrap());
    }
    if s[consumed] != b']' {
        println!(
            "expected ']', got '{}', input: {}",
            s[consumed] as char,
            from_utf8(s).unwrap()
        );
        return None;
    }
    consumed += 1;

    //println!( "return {:?} remaining: {}", rv, from_utf8(&s[consumed..]).unwrap());
    return Some((ListOrInt::List(rv), consumed));
}

fn compare_int_int(n1: i32, n2: i32) -> Option<bool> {
    if n1 < n2 {
        Some(true)
    } else if n1 > n2 {
        Some(false)
    } else {
        None
    }
}

fn compare_list_list(l1: &Vec<ListOrInt>, l2: &Vec<ListOrInt>) -> Option<bool> {
    for i in 0..l1.len().min(l2.len()) {
        if let Some(res) = compare(&l1[i], &l2[i]) {
            return Some(res);
        }
    }
    if l1.len() < l2.len() {
        return Some(true);
    }
    if l2.len() < l1.len() {
        return Some(false);
    }
    None
}

fn compare(l1: &ListOrInt, l2: &ListOrInt) -> Option<bool> {
    let rv = match l1 {
        Int(int1) => match l2 {
            Int(int2) => compare_int_int(*int1, *int2),
            List(list2) => compare_list_list(&vec![Int(*int1)], list2),
        },
        List(list1) => match l2 {
            Int(int2) => compare_list_list(list1, &vec![Int(*int2)]),
            List(list2) => compare_list_list(list1, list2),
        },
    };
    rv
}

fn solve_part1(f: &str) -> i32 {
    //let mut packets = vec![];
    fs::read_to_string(f)
        .unwrap()
        .split("\n\n")
        .map(|ls| ls.lines().collect::<Vec<&str>>())
        .filter(|ls| ls.len() == 2)
        .map(|ls| {
            let l1 = consume_list(&ls[0].as_bytes()).unwrap().0;
            let l2 = consume_list(&ls[1].as_bytes()).unwrap().0;
            let rv = compare(&l1, &l2);
            println!("a<b: {:?}: a={} b={}", rv, ls[0], ls[1]);
            rv
        })
        .enumerate()
        .filter(|(_, res)| res.is_none() || res.unwrap())
        .map(|(idx, _)| idx + 1)
        .sum::<usize>() as i32
}

fn solve_part2(f: &str) -> i32 {
    let mut l = fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .map(|ls| consume_list(&ls.as_bytes()).unwrap().0)
        .collect::<Vec<ListOrInt>>();
    let p1 = consume_list(b"[[2]]").unwrap().0;
    let p2 = consume_list(b"[[6]]").unwrap().0;
    l.append(&mut vec![
        p1.clone(),
        p2.clone(),
    ]);
    l.sort_by(|p1, p2| {
        if let Some(res) = compare(p1, p2) {
            if res {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Less
        }
    });
    l.iter()
        .enumerate()
        .filter(|(_, v)| **v == p1 || **v == p2)
        .map(|(i, _)| i + 1)
        .product::<usize>() as i32
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
