use std::{cell::RefCell, fs, rc::Rc};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list() {
        let test_cases = [
            (vec![1], vec![vec![1]]),
            //vec![vec![0], vec![0], vec![0]),
            (vec![2, 1, 0], vec![vec![2, 1, 0]]),
            (vec![-2, 1, 0], vec![vec![-2, 1, 0]]),
            // wrong - -3 needs to be take out, next line is what's expected:
            // (vec![0, -3, 5], vec![vec![0, -3, 5], vec![0,-3,5]]),
            (vec![0, -3, 5], vec![vec![0, -3, 5], vec![5,-3,0]]),
            (
                vec![0, 1, 2],
                vec![vec![0, 1, 2], vec![0, 2, 1], vec![0, 2, 1]],
            ),
            //vec![vec![1 , -1, -2], vec![-1, 1, -2], vec![1,-1,-2], vec![1,-1,-2]],
            (
                vec![1, 2, -3, 3, -2, 0, 4],
                vec![
                    vec![2, 1, -3, 3, -2, 0, 4],
                    vec![1, -3, 2, 3, -2, 0, 4],
                    vec![1, 2, 3, -2, -3, 0, 4],
                    vec![1, 2, -2, -3, 0, 3, 4],
                    vec![1, 2, -3, 0, 3, 4, -2],
                    vec![1, 2, -3, 0, 3, 4, -2],
                    vec![1, 2, -3, 4, 0, 3, -2],
                ],
            ),
        ];
        fn check(l1: &[i32], l2: &[i32]) -> bool {
            if l1.len() != l2.len() {
                return false;
            }
            let mut tmp = vec![];
            tmp.extend_from_slice(l2);
            for _ in 0..l1.len() {
                if tmp == l1 {
                    return true;
                }
                tmp.rotate_left(1);
            }
            return false;
        }
        for tc in test_cases {
            let s = &tc.0;
            for i in 0..tc.1.len() {
                let got = move_by_list(s, i+1);
                let expected = &tc.1[i];
                println!("{:?}", got);
                if !check(&got, &expected) {
                    panic!(
                        "fail on {:?} at iteration {}, {:?} != {:?}",
                        s,
                        i + 1,
                        got,
                        expected
                    );
                }
            }
        }
    }
    #[test]
    fn test_move_n() {
        let l = read_input("test.txt");
        assert_eq!(move_n(&l, 1), [2, 1, -3, 3, -2, 0, 4]);
        assert_eq!(move_n(&l, 2), [1, -3, 2, 3, -2, 0, 4]);
        assert_eq!(move_n(&l, 3), [1, 2, 3, -2, -3, 0, 4]);
        assert_eq!(move_n(&l, 4), [1, 2, -2, -3, 0, 3, 4]);
        assert_eq!(move_n(&l, 5), [1, 2, -3, 0, 3, 4, -2]);
        assert_eq!(move_n(&l, 6), [1, 2, -3, 0, 3, 4, -2]);
        assert_eq!(move_n(&l, 7), [1, 2, -3, 4, 0, 3, -2]);
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 3);
        assert_eq!(solve_part1("input.txt"), 13522);
    }
    #[test]
    fn test_part2() {
        //assert_eq!(solve_part1("test.txt"), -1);
    }
}

fn move_by_list(s: &[i32], n: usize) -> Vec<i32> {
    let l = from_slice(&s);
    let orig = as_ordered_vec(&l);
    for i in 0..n {
        shift(&orig[i].1);
    }
    let fin = as_vec(&l);
    assert_eq!(s.len(), fin.len());
    return fin;
    let zero_position = s.iter().position(|x| *x == 0).unwrap();
    let mut sum = 0;
    let mut zero = orig[zero_position].1.clone();

    for _ in 0..3 {
        for _ in 0..1000 {
            zero = next_right(&zero);
        }
        println!("+1000 = {}", zero.borrow().elem);
        sum += zero.borrow().elem;
    }

    println!(
        "sum: {} zero position was {} with {} {}, now it has {}",
        sum,
        zero_position,
        orig[zero_position].0,
        orig[zero_position].1.borrow().elem,
        fin[zero_position]
    );
    fin
}

struct ListElement {
    elem: i32,
    id: usize,
    left: Option<Rc<RefCell<ListElement>>>,
    right: Option<Rc<RefCell<ListElement>>>,
}
fn as_ordered_vec(x: &Rc<RefCell<ListElement>>) -> Vec<(usize, Rc<RefCell<ListElement>>)> {
    let mut rv = vec![];
    let mut cur = x.clone();
    loop {
        if cur.borrow().id == 0 {
            break;
        }
        cur = next_right(&cur);
    }
    let mut n = 0;
    loop {
        rv.push((n, cur.clone()));
        n += 1;
        cur = next_right(&cur);
        if cur.borrow().id == 0 {
            break;
        }
    }
    rv
}
fn as_vec(x: &Rc<RefCell<ListElement>>) -> Vec<i32> {
    let mut rv = vec![];
    let mut cur = x.clone();
    loop {
        if cur.borrow().id == 0 {
            break;
        }
        cur = next_right(&cur);
    }
    loop {
        rv.push(cur.borrow().elem);
        cur = next_right(&cur);
        if cur.borrow().id == 0 {
            break;
        }
    }
    rv
}
fn print_list(x: &Rc<RefCell<ListElement>>) {
    println!("{:?}", as_vec(x));
}

fn shift(x: &Rc<RefCell<ListElement>>) {
    let dir = x.borrow_mut().elem.signum();
    if dir == 0 {
        return;
    }
    let n = x.borrow().elem.abs();

    let a = x.borrow().left.as_ref().unwrap().clone();
    let b = x.borrow().right.as_ref().unwrap().clone();
    if x.borrow().id == a.borrow().id {
        // single element list
        return;
    }
    assert_ne!(x.borrow().id, b.borrow().id);
    assert_ne!(a.borrow().id, b.borrow().id);

    // Remove x then cycle around.
    // A->X becomes A->B
    a.borrow_mut().right = Some(b.clone());
    // B->X becomes B->A
    b.borrow_mut().left = Some(a.clone());

    let mut cur = x.clone();
    for _ in 0..n {
        cur = if dir > 0 {
            // right move
            let y = cur.borrow().right.as_ref().unwrap().clone();
            y
        } else {
            // left move
            let y = cur.borrow().left.as_ref().unwrap().clone();
            y
        }
    }

    let (cur, c) = if x.borrow().elem > 0 {
        (cur.clone(), cur.borrow().right.as_ref().unwrap().clone())
    } else {
        (cur.borrow().left.as_ref().unwrap().clone(), cur.clone())
    };
    // A->X->B->...->Cur->C becomes A->B->...->Cur->X->C
    /*
    println!(
        "inserting {} between {} and {}",
        x.borrow().elem,
        cur.borrow().elem,
        c.borrow().elem
    );
    println!(
        "...-> {} -> {} -> {} ->...-> {} -> {} ->... becomes ",
        a.borrow().elem,
        x.borrow().elem,
        b.borrow().elem,
        cur.borrow().elem,
        c.borrow().elem
    );
     */

    assert_ne!(cur.borrow().id, c.borrow().id);
    if x.borrow().id == cur.borrow().id {
        a.borrow_mut().right = Some(x.clone());
        b.borrow_mut().left = Some(x.clone());
        // x and cur are the same
        return;
    }
    if x.borrow().id == c.borrow().id {
        a.borrow_mut().right = Some(x.clone());
        b.borrow_mut().left = Some(x.clone());
        // x and c are the same
        return;
    }

    // Cur->C becomes Cur->X
    cur.borrow_mut().right = Some(x.clone());
    // X->B becomes X->C
    x.borrow_mut().right = Some(c.clone());
    // X->A becomes X->Cur
    x.borrow_mut().left = Some(cur.clone());
    // C->Cur becomes C->X
    c.borrow_mut().left = Some(x.clone());

    /*
    println!(
        "...-> {} -> {} ->...-> {} -> {} -> {} ->...",
        a.borrow().elem,
        b.borrow().elem,
        cur.borrow().elem,
        x.borrow().elem,
        c.borrow().elem
    );
     */
}

fn from_slice(l: &[i32]) -> Rc<RefCell<ListElement>> {
    let mut first_element = Rc::new(RefCell::new(ListElement {
        elem: l[0],
        id: 0,
        left: None,
        right: None,
    }));
    first_element.borrow_mut().left = Some(first_element.clone());
    first_element.borrow_mut().right = Some(first_element.clone());

    let mut next_element = first_element.clone();
    for i in 1..l.len() {
        next_element = insert_right(&next_element, l[i], i);
    }

    first_element
}
fn next_right(elem: &Rc<RefCell<ListElement>>) -> Rc<RefCell<ListElement>> {
    elem.borrow().right.as_ref().unwrap().clone()
}
fn insert_right(elem: &Rc<RefCell<ListElement>>, n: i32, id: usize) -> Rc<RefCell<ListElement>> {
    let a = elem.clone();
    let b = elem.borrow().right.as_ref().unwrap().clone();

    let x = Rc::new(RefCell::new(ListElement {
        elem: n,
        id,
        left: None,
        right: None,
    }));

    x.borrow_mut().left = Some(a.clone());
    x.borrow_mut().right = Some(b.clone());
    a.borrow_mut().right = Some(x.clone());
    b.borrow_mut().left = Some(x.clone());

    x
}

fn read_input(f: &str) -> Vec<i32> {
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
fn solve_part1(f: &str) -> i32 {
    let input = read_input(f);
    let l = move_by_list(&input, input.len());
    let zero_position = l.iter().position(|x| *x == 0).unwrap();

    l[(zero_position + 1000) % l.len()]
        + l[(zero_position + 2000) % l.len()]
        + l[(zero_position + 3000) % l.len()]
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
    println!("part 1: {}", solve_part1("test.txt"));
    println!("part 1: {}", solve_part1("input.txt"));
    //println!("part 2: {}", solve_part2("input.txt"));
}
