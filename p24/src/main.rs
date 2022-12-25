use std::fs;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bliz() {
        let b = Bliz::new(2, 1, 5);
        assert!(b.at(0, 2));
        assert!(b.at(1, 3));
        assert!(b.at(0, -3));
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 18);
        assert_eq!(solve_part1("input.txt"), 301);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("test.txt"), 54);
        assert_eq!(solve_part2("input.txt"), 859);
    }
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

impl Map {
    fn height(&self) -> usize {
        self.rows.len()
    }
    fn width(&self) -> usize {
        self.cols.len()
    }
    fn parse(inp: &[Vec<u8>]) -> Map {
        let height = inp.len() as u8;
        let width = inp[0].len() as u8;

        let mut rows = vec![vec![]; 2 + height as usize];
        let mut cols = vec![vec![]; 2 + width as usize];
        inp.iter().enumerate().for_each(|(row_num, in_row)| {
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
        Pos(self.rows.len() - 2, self.cols.len() - 2)
    }
    fn at1(&self, n: usize, p: Pos) -> bool {
        for bliz in &self.rows[p.0] {
            if bliz.at(n, p.1 as isize - 1) {
                return false;
            }
        }
        for bliz in &self.cols[p.1] {
            if bliz.at(n, p.0 as isize - 1) {
                return false;
            }
        }
        true
    }
}

fn solve2(m: &Map, starting_n: usize, from: Pos, to: Pos) -> usize {
    let mut can_be = vec![vec![vec![10000; m.width()]; m.height()]; 1];
    let mut n = 1;
    loop {
        can_be.push(vec![vec![10000; m.width()]; m.height()]);
        can_be[n][from.0][from.1] = if m.at1(n + starting_n - 1, from) {
            n
        } else {
            10000
        };
        for r in 1..(m.height() - 1) {
            for c in 1..(m.width() - 1) {
                if Pos(r, c) == from {
                    continue;
                }
                can_be[n][r][c] = if m.at1(n + starting_n - 1, Pos(r, c)) {
                    [
                        can_be[n - 1][r][c],
                        can_be[n - 1][r - 1][c],
                        can_be[n - 1][r + 1][c],
                        can_be[n - 1][r][c - 1],
                        can_be[n - 1][r][c + 1],
                    ]
                    .into_iter()
                    .min()
                    .unwrap()
                } else {
                    10000
                };
                if can_be[n][r][c] < 10000 {
                    can_be[n][r][c] += 1;
                }
            }
        }
        /*
        println!("Minute {n}");
        for r in 1..(m.height() - 1) {
            for c in 1..(m.width() - 1) {
                print!(
                    "{:3}",
                    if can_be[n][r][c] == 10000 {
                        99
                    } else {
                        can_be[n][r][c]
                    }
                );
            }
            print!("   |   ");
            for c in 1..(m.width() - 1) {
                print!("{} ", if m.at1(n, Pos(r - 1, c - 1)) { 1 } else { 0 });
            }
            println!();
        }
         */
        if can_be[n][to.0][to.1] < 10000 {
            break;
        }
        n = n + 1;
    }
    n + starting_n
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
    solve2(&m, 1, Pos(1, 1), m.bottom_right())
}

fn solve_part2(f: &str) -> usize {
    let m = read_input(f);
    let n1 = 1;
    let n1 = solve2(&m, n1, Pos(1, 1), m.bottom_right());
    let n1 = solve2(&m, n1, m.bottom_right(), Pos(1, 1));
    let n1 = solve2(&m, n1, Pos(1, 1), m.bottom_right());
    n1
}

fn main() {
    println!("part 1: {}", solve_part1("test.txt"));
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("test.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
