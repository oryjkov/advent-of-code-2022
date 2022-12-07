use itertools::Itertools;
use std::{cell::RefCell, fs, rc::Rc};
struct Dir {
    name: String,
    files: Vec<(String, usize)>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}
impl Dir {
    fn new(name: &str, parent: Option<Rc<RefCell<Dir>>>) -> Self {
        Dir {
            name: name.to_string(),
            files: vec![],
            dirs: vec![],
            parent: parent,
        }
    }
}

fn flatten(d: &Rc<RefCell<Dir>>) -> Vec<(String, usize)> {
    let name = d.borrow().name.clone();
    let files_size = d
        .borrow()
        .files
        .iter()
        .map(|(_, size)| *size)
        .sum::<usize>();
    d.borrow()
        .dirs
        .iter()
        .map(flatten)
        .fold(vec![(name, files_size)], |mut acc, mut item| {
            // The first element in each item is the actual size of the subdir.
            // Combine it by updating our size and appending the list of subdirs.
            acc[0].1 += item[0].1;
            acc.append(&mut item);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let root = build("test.txt");
        assert_eq!(95437, solve_p1(&root));
    }
    #[test]
    fn test_p2() {
        let root = build("test.txt");
        assert_eq!(24933642, solve_p2(&root));
    }
}

fn build(f: &str) -> Rc<RefCell<Dir>> {
    let root = Rc::new(RefCell::new(Dir::new("/", None)));
    let mut d = root.clone();

    let mut flag = false;
    fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .group_by(|&l| {
            if l.chars().next().unwrap() == '$' {
                flag = !flag;
            }
            flag
        })
        .into_iter()
        .map(|(_, mut group)| {
            let cmd = group
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            match cmd[1] {
                "cd" => {
                    d = match cmd[2] {
                        ".." => {
                            if let Some(p) = &d.borrow().parent {
                                p.clone()
                            } else {
                                d.clone()
                            }
                        }
                        "/" => root.clone(),
                        _ => {
                            let mut new_d = d.clone();
                            for subdir in &d.borrow().dirs {
                                if subdir.borrow().name == cmd[2] {
                                    new_d = subdir.clone();
                                    break;
                                }
                            }
                            new_d
                        }
                    };
                }
                "ls" => group.for_each(|e| {
                    let mut parts = e.split_whitespace();
                    let word1 = parts.next().unwrap();
                    if word1 == "dir" {
                        let new_dir = Rc::new(RefCell::new(Dir::new(
                            parts.next().unwrap(),
                            Some(d.clone()),
                        )));
                        d.borrow_mut().dirs.push(new_dir);
                    } else {
                        d.borrow_mut().files.push((
                            parts.next().unwrap().to_string(),
                            word1.parse::<usize>().unwrap(),
                        ));
                    }
                }),
                _ => {}
            }
        })
        .count();
    root
}

fn solve_p1(root: &Rc<RefCell<Dir>>) -> usize{
    flatten(root)
        .iter()
        .map(|(_, size)| *size)
        .filter(|&d| d <= 100_000)
        .sum::<usize>()
}

fn solve_p2(root: &Rc<RefCell<Dir>>) -> usize {
    let all = flatten(root);
    let total_size = all[0].1;
    let to_clean_up = 30000000 - (70000000 - total_size);

    all[1..].iter().
        map(|(_, size)| *size)
        .filter(|&d| d >= to_clean_up).min().unwrap()
}

fn main() {
    let f = "input.txt";
    let root = build(f);
    println!("answer: {}", solve_p1(&root));
    println!("answer2: {}", solve_p2(&root));
}
