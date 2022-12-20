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
            (vec![0, -3, 5], vec![vec![0, -3, 5], vec![5, -3, 0]]),
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
        fn check(l1: &[i64], l2: &[i64]) -> bool {
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
                let got = mix_partial(s, i + 1);
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
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 3);
        assert_eq!(solve_part1("input.txt"), 13522);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("test.txt"), 1623178306);
        assert_eq!(solve_part2("input.txt"), 17113168880158);
    }
}
fn mix_n(s: &[i64], num: usize) -> Vec<i64> {
    let l = from_slice(&s);
    let orig = as_ordered_vec(&l);
    for _ in 0..num {
        for i in 0..s.len() {
            shift(&orig[i].1);
        }
    }
    let fin = as_vec(&l);
    assert_eq!(s.len(), fin.len());
    return fin;
}

fn mix_partial(s: &[i64], n: usize) -> Vec<i64> {
    let l = from_slice(&s);
    let orig = as_ordered_vec(&l);
    for i in 0..n {
        shift(&orig[i].1);
    }
    let fin = as_vec(&l);
    assert_eq!(s.len(), fin.len());
    return fin;
}

struct ListElement {
    elem: i64,
    id: usize,
    len: usize,
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
fn as_vec(x: &Rc<RefCell<ListElement>>) -> Vec<i64> {
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
    if x.borrow().elem == 0 {
        return;
    }
    if x.borrow().len == 1 {
        return;
    }
    // modulo len-1 since that's what works.
    let len = x.borrow().len - 1;
    let n = x.borrow().elem.rem_euclid(x.borrow().len as i64 - 1);
    if n == 0 {
        return;
    }

    let a = x.borrow().left.as_ref().unwrap().clone();
    let b = x.borrow().right.as_ref().unwrap().clone();

    // Already checked for a single element list.
    assert_ne!(x.borrow().id ,a.borrow().id);
    assert_ne!(x.borrow().id, b.borrow().id);
    assert_ne!(a.borrow().id, b.borrow().id);

    let mut cur = x.clone();
    for _ in 0..n {
        cur = next_right(&cur);
    }
    let c = cur.borrow().right.as_ref().unwrap().clone();

    // A->X becomes A->B
    a.borrow_mut().right = Some(b.clone());
    // B->X becomes B->A
    b.borrow_mut().left = Some(a.clone());

    // A->X->B->...->Cur->C becomes A->B->...->Cur->X->C
    assert_ne!(cur.borrow().id, c.borrow().id);
    assert_ne!(x.borrow().id, cur.borrow().id);
    assert_ne!(x.borrow().id, c.borrow().id);
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
}

fn from_slice(l: &[i64]) -> Rc<RefCell<ListElement>> {
    let mut first_element = Rc::new(RefCell::new(ListElement {
        elem: l[0],
        id: 0,
        len: l.len(),
        left: None,
        right: None,
    }));
    first_element.borrow_mut().left = Some(first_element.clone());
    first_element.borrow_mut().right = Some(first_element.clone());

    let mut next_element = first_element.clone();
    for i in 1..l.len() {
        next_element = insert_right(&next_element, l[i], i, l.len());
    }

    first_element
}
fn next_right(elem: &Rc<RefCell<ListElement>>) -> Rc<RefCell<ListElement>> {
    elem.borrow().right.as_ref().unwrap().clone()
}
fn insert_right(
    elem: &Rc<RefCell<ListElement>>,
    n: i64,
    id: usize,
    len: usize,
) -> Rc<RefCell<ListElement>> {
    let a = elem.clone();
    let b = elem.borrow().right.as_ref().unwrap().clone();

    let x = Rc::new(RefCell::new(ListElement {
        elem: n,
        id,
        len,
        left: None,
        right: None,
    }));

    x.borrow_mut().left = Some(a.clone());
    x.borrow_mut().right = Some(b.clone());
    a.borrow_mut().right = Some(x.clone());
    b.borrow_mut().left = Some(x.clone());

    x
}

fn read_input(f: &str) -> Vec<i64> {
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
fn solve_part1(f: &str) -> i64 {
    let input = read_input(f);
    let l = mix_partial(&input, input.len());
    let zero_position = l.iter().position(|x| *x == 0).unwrap();

    l[(zero_position + 1000) % l.len()]
        + l[(zero_position + 2000) % l.len()]
        + l[(zero_position + 3000) % l.len()]
}

fn solve_part2(f: &str) -> i64 {
    let input: Vec<i64> = read_input(f).iter().map(|x| x * 811589153).collect();
    let l = mix_n(&input, 10);
    let zero_position = l.iter().position(|x| *x == 0).unwrap();

    l[(zero_position + 1000) % l.len()]
        + l[(zero_position + 2000) % l.len()]
        + l[(zero_position + 3000) % l.len()]
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
