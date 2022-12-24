use std::fs;
use std::ops::{Index, IndexMut};
use std::str::from_utf8;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply() {
        use Dir::*;
        assert_eq!(Right.apply(Pos(1, 1), Pos(1, 1)), None);
        assert_eq!(Down.apply(Pos(2, 2), Pos(2, 2)), None);
        assert_eq!(Up.apply(Pos(2, 2), Pos(2, 2)).unwrap(), Pos(1, 2));
        assert_eq!(Left.apply(Pos(2, 2), Pos(2, 2)).unwrap(), Pos(2, 1));
        assert_eq!(Same.apply(Pos(2, 2), Pos(2, 2)).unwrap(), Pos(2, 2));
    }
    #[test]
    fn test_bliz() {
        let b = Bliz::new(2, 1, 5);
        assert!(b.at(0, 2));
        assert!(b.at(1, 3));
        assert!(b.at(0, -3));
    }
    #[test]
    fn test_at() {
        #[rustfmt::skip]
         let inp = [
            ">..".as_bytes(),
            "...".as_bytes(),
            "...".as_bytes(),
        ];
        let m = Map::parse(&inp);
        assert_eq!(m.at(0, Pos(2, 2)), [0; 5]);
        assert_eq!(m.at(0, Pos(0, 0)), [1, 0, 0, 0, 0]);
        assert_eq!(m.at(1, Pos(0, 0)), [0, 0, 0, 0, 1]);
        assert_eq!(m.at(1, Pos(1, 1)), [0, 1, 0, 0, 0]);

        #[rustfmt::skip]
        let inp = [
            ".v.".as_bytes(),
            "...".as_bytes(),
            "...".as_bytes(),
        ];
        println!("input 2");
        let m = Map::parse(&inp);
        assert_eq!(m.at(0, Pos(0, 0)), [0, 0, 0, 0, 1]);
        assert_eq!(m.at(1, Pos(1, 1)), [1, 0, 0, 0, 0]);
        assert_eq!(m.at(0, Pos(1, 1)), [0, 1, 0, 0, 0]);

        #[rustfmt::skip]
        let inp = [
            ".v..".as_bytes(),
            "<.<.".as_bytes(),
            ".^..".as_bytes(),
        ];
        println!("input 2");
        let m = Map::parse(&inp);
        assert_eq!(m.at(0, Pos(0, 0)), [0, 0, 0, 1, 1]);
        assert_eq!(m.at(1, Pos(1, 1)), [3, 0, 0, 0, 0]);
        assert_eq!(m.at(0, Pos(1, 1)), [0, 1, 1, 1, 1]);
    }
    #[test]
    fn test_part1() {
        //assert_eq!(solve_part1("test.txt"), _);
    }
    #[test]
    fn test_part2() {}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Pos(usize, usize);
#[derive(Copy, Clone)]
struct Step(isize, isize);
#[derive(Copy, Clone)]

struct Bliz {
    start: usize,
    step: i8,
    len: u8,
}
impl Bliz {
    fn new(start: usize, step: i8, len: u8) -> Self {
        Bliz { start, step, len }
    }
    fn at(&self, n: usize, c: isize) -> bool {
        c.rem_euclid(self.len as isize) as usize
            == (self.start as i32 + n as i32 * (self.step as i32)).rem_euclid(self.len as i32)
                as usize
    }
}

struct Map {
    rows: Vec<Vec<Bliz>>,
    cols: Vec<Vec<Bliz>>,
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Same = 0,
    Up = 1,
    Left = 2,
    Down = 3,
    Right = 4,
}
impl Dir {
    fn apply(&self, p: Pos, max_pos: Pos) -> Option<Pos> {
        use Dir::*;
        match self {
            Same => Some(p),
            Up => {
                if p.0 == 0 {
                    None
                } else {
                    Some(Pos(p.0 - 1, p.1))
                }
            }
            Left => {
                if p.1 == 0 {
                    None
                } else {
                    Some(Pos(p.0, p.1 - 1))
                }
            }
            Down => {
                if p.0 == max_pos.0 {
                    None
                } else {
                    Some(Pos(p.0 + 1, p.1))
                }
            }
            Right => {
                if p.1 == max_pos.1 {
                    None
                } else {
                    Some(Pos(p.0, p.1 + 1))
                }
            }
        }
    }
}
impl<T> Index<Dir> for [T; 5] {
    type Output = T;
    fn index(&self, r: Dir) -> &T {
        &self[r as usize]
    }
}

impl<T> IndexMut<Dir> for [T; 5] {
    fn index_mut(&mut self, r: Dir) -> &mut T {
        &mut self[r as usize]
    }
}

impl Map {
    fn parse(inp: &[Vec<u8>]) -> Map {
        let height = inp.len() as u8;
        let width = inp[0].len() as u8;

        let mut rows = vec![vec![]; 2 + height as usize];
        let mut cols = vec![vec![]; 2 + width as usize];
        inp.iter().enumerate().for_each(|(row_num, in_row)| {
            println!("#{}#", from_utf8(in_row).unwrap());
            in_row
                .iter()
                .enumerate()
                .for_each(|(col_num, in_char)| match *in_char {
                    b'>' => rows[row_num + 1].push(Bliz::new(col_num, 1, width)),
                    b'<' => rows[row_num + 1].push(Bliz::new(col_num, -1, width)),
                    b'^' => cols[col_num + 1].push(Bliz::new(row_num, -1, height)),
                    b'v' => cols[col_num + 1].push(Bliz::new(row_num, 1, height)),
                    b'.' => (),
                    _ => panic!("wrong input char"),
                });
        });
        let m = Map { rows, cols };
        println!(
            "height: {}, width: {}, bottom: {:?}",
            height,
            width,
            m.bottom_right()
        );
        m
    }
    fn bottom_right(&self) -> Pos {
        Pos(self.rows.len() - 3, self.cols.len() - 3)
    }
    fn at_hor(&self, rows: &[usize], n: usize, c: usize, rv: &mut [usize; 5]) {
        for bliz in self.rows[rows[0]].iter() {
            if bliz.at(n, c as isize) {
                rv[Dir::Up] += 1;
            }
        }
        for bliz in self.rows[rows[2]].iter() {
            if bliz.at(n, c as isize) {
                rv[Dir::Down] += 1;
            }
        }
        for bliz in self.rows[rows[1]].iter() {
            if bliz.at(n, c as isize) {
                rv[Dir::Same] += 1;
            } else if bliz.at(n, c as isize - 1) {
                rv[Dir::Left] += 1;
            } else if bliz.at(n, c as isize + 1) {
                rv[Dir::Right] += 1;
            }
        }
    }
    fn at_ver(&self, cols: &[usize], n: usize, c: usize, rv: &mut [usize; 5]) {
        for bliz in self.cols[cols[0]].iter() {
            if bliz.at(n, c as isize) {
                rv[Dir::Left] += 1;
            }
        }
        for bliz in self.cols[cols[2]].iter() {
            if bliz.at(n, c as isize) {
                rv[Dir::Right] += 1;
            }
        }
        for bliz in self.cols[cols[1]].iter() {
            if bliz.at(n, c as isize) {
                rv[Dir::Same] += 1;
            } else if bliz.at(n, c as isize - 1) {
                rv[Dir::Up] += 1;
            } else if bliz.at(n, c as isize + 1) {
                rv[Dir::Down] += 1;
            }
        }
    }
    fn at(&self, n: usize, p: Pos) -> [usize; 5] {
        //println!("at {:?}", p);
        let mut rv = [0; 5];
        let (row, col) = (p.0, p.1);

        self.at_ver(&[col, col + 1, col + 2], n, row, &mut rv);
        //println!("at ver: {:?}", rv);
        self.at_hor(&[row, row + 1, row + 2], n, col, &mut rv);
        //println!("together: {:?}", rv);

        rv
    }
}

type Path = Vec<Dir>;

fn solve(m: &Map) -> usize {
    let mut path = vec![];
    let mut min_len = 100000;
    loop {
        let n = path.len();
        let neibs = m.at(n + 1, Pos(0, 0));
        if neibs[Dir::Down] == 0 {
            println!("starting at n={n}");
            walk(m, &mut path, &mut min_len, Pos(0, 0));
            println!("starting at n={n}, min len: {}", min_len);
        }
        path.push(Dir::Same);
        if n > 3 {
            break;
        }
    }
    min_len + 1
}

fn walk(m: &Map, path: &mut Path, min_len: &mut usize, pos: Pos) -> bool {
    use Dir::*;
    let n = path.len();
    if n >= *min_len {
        return false;
    }
    let br = m.bottom_right();
    if pos == br {
        *min_len = n;
        println!("len: {}", path.len());
        return true;
    }
    let dirs = if br.1 - pos.1 > br.0 - pos.0 {
        [Right, Down, Same, Up, Left]
    } else {
        [Down, Right, Same, Up, Left]
    };
    let neibs = m.at(n + 1, pos);
    for dir in dirs {
        if neibs[dir] > 0 {
            continue;
        }
        if let Some(new_pos) = dir.apply(pos, m.bottom_right()) {
            path.push(dir);
            walk(m, path, min_len, new_pos);
            path.pop();
        }
    }
    return false;
}

fn read_input(f: &str) -> Map {
    Map::parse(
        &fs::read_to_string(f)
            .unwrap()
            .lines()
            .filter(|l| l.len() > 0)
            .skip(1)
            .map(|l| {
                l.as_bytes()
                    .iter()
                    .skip(1)
                    .take_while(|b| **b != b'#')
                    .map(|b| *b)
                    .collect::<Vec<u8>>()
            })
            .take_while(|bts| bts.len() > 1)
            .collect::<Vec<Vec<u8>>>(),
    )
}

fn solve_part1(f: &str) -> usize {
    let m = read_input(f);
    solve(&m)
}

fn solve_part2(f: &str) -> i32 {
    fs::read_to_string(f).unwrap().lines().count();
    -1
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
