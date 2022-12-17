use std::{collections::VecDeque, fs, time::{Instant, Duration}};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 3068);
        assert_eq!(solve_part1("input.txt"), 3224);
    }
    #[test]
    fn test_part2() {
        //assert_eq!(solve_part2("test.txt"), 1514285714288);
    }
}

fn rocks() -> Vec<Rock> {
    vec![
        Rock {
            bm: vec![vec![b'#', b'#', b'#', b'#']],
        },
        Rock {
            bm: vec![
                vec![b'.', b'#', b'.'],
                vec![b'#', b'#', b'#'],
                vec![b'.', b'#', b'.'],
            ],
        },
        Rock {
            bm: vec![
                vec![b'.', b'.', b'#'],
                vec![b'.', b'.', b'#'],
                vec![b'#', b'#', b'#'],
            ],
        },
        Rock {
            bm: vec![vec![b'#'], vec![b'#'], vec![b'#'], vec![b'#']],
        },
        Rock {
            bm: vec![vec![b'#', b'#'], vec![b'#', b'#']],
        },
    ]
}

const NUM_ROCKS: usize = 2022;
const N2: usize = 1_000_000_000_000;

fn solve(f: &str, n: usize) -> i64 {
    let jets = fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>()[0]
        .as_bytes()
        .iter()
        .map(|c| if *c == b'<' { -1 } else { 1 })
        .collect::<Vec<isize>>();
    let rocks = rocks();
    let mut field = Field::new();
    let mut jets_iter = jets.iter().cycle();
    let start = Instant::now();
    for (rock_num, rock) in rocks.iter().cycle().enumerate() {
        if rock_num >= n {
            break;
        }
        field.grow(rock.height() + 3);
        let mut top_x = 2;
        let mut top_y = 0;
        for offset in &mut jets_iter {
            // show the rock
            //field.try_place(rock, top_x, top_y).show();

            if field.check(rock, top_x + offset, top_y) {
                top_x += offset;
                // show the rock
                //field.try_place(rock, top_x, top_y).show();
            }
            let should_stop = !field.check(rock, top_x, top_y + 1);
            if should_stop {
                break;
            }
            top_y += 1;
        }

        field.place(rock, top_x, top_y);
        //field.show();
        //field.show();
        if field.rows.len() > 1_000_000 {
            field.shrink_bottom();
        }
        if rock_num % 10_000_000 == 1_000_000 {
            let mrps = (rock_num as f64) / start.elapsed().as_secs_f64() / 1e6;
            let will_take = Duration::from_secs_f64(((n-rock_num) as f64 / mrps) / 1e6);
            println!("{}, Mrps: {:.2}, will take: {:?}", rock_num, mrps, will_take );
        }
    }
    field.height() as i64
}

fn solve_part1(f: &str) -> i64 {
    solve(f, NUM_ROCKS)
}
fn solve_part2(f: &str) -> i64 {
    solve(f, N2)
}

#[derive(Clone, Debug)]
struct Rock {
    bm: Vec<Vec<u8>>,
}
impl Rock {
    fn height(&self) -> usize {
        self.bm.len()
    }
    fn tentative(&self) -> Self {
        let mut rv = self.clone();
        rv.bm.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|r| {
                if !cell_empty(*r) {
                    *r = b'@'
                }
            })
        });
        rv
    }
}

fn cell_empty(cell: u8) -> bool {
    cell == b'.'
}

#[derive(Clone)]
struct Field {
    rows: Vec<[u8; 9]>,
    cleared: usize,
}

const EMPTY_ROW: [u8; 9] = [b'|', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'|'];
const BOTTOM_ROW: [u8; 9] = [b'+', b'-', b'-', b'-', b'-', b'-', b'-', b'-', b'+'];
impl Field {
    fn new() -> Self {
        Field {
            rows: vec![BOTTOM_ROW],
            cleared: 0,
        }
    }

    fn height(&self) -> usize {
        self.rows.len() - 1 + self.cleared
    }
    // Checks if placing a rock at x and with top at y is valid.
    fn check(&self, rock: &Rock, top_x: isize, top_y: usize) -> bool {
        rock.bm
            .iter()
            .enumerate()
            .try_fold(false, |_, (rock_y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| !cell_empty(**cell))
                    .try_fold(false, |_, (rock_x, _)| {
                        let x = rock_x as isize + top_x;
                        let y = rock_y + top_y;
                        if self.get_cell(x, y) {
                            None
                        } else {
                            Some(true)
                        }
                    })
            })
            .is_some()
    }
    // Returns true if there is a rock at this position.
    // If the row does not exist, then there is no rock there.
    fn get_cell(&self, x: isize, y: usize) -> bool {
        let actual_y = self.rows.len() - y - 1;
        if x <= -1 {
            return true;
        }
        let actual_x = (x + 1) as usize;
        if actual_y >= self.rows.len() {
            false
        } else if actual_x >= self.rows[0].len() {
            true
        } else {
            !cell_empty(self.rows[actual_y][actual_x])
        }
    }
    fn put_cell(&mut self, x: isize, y: usize, cell: u8) {
        let actual_y = self.rows.len() - y - 1;
        let actual_x = (x + 1) as usize;
        self.rows[actual_y][actual_x] = cell;
    }

    fn grow(&mut self, n: usize) {
        (0..n).for_each(|_| self.rows.push(EMPTY_ROW));
    }
    fn shrink_bottom(&mut self) {
        let mut can_clear = self.rows.len();
        let mut b = vec![];
        for col in 0..7 {
            let mut y = self.rows.len() - 1;
            while cell_empty(self.rows[y][col + 1]) {
                y -= 1;
            }
            can_clear = can_clear.min(y);
            b.push(y)
        }
        //println!( "max: {}, {:?}, can clear {} bottom rows", can_clear, b, can_clear);
        self.rows = self.rows.split_off(can_clear);
        self.cleared += can_clear;
    }
    fn shrink(&mut self) {
        while self.rows[self.rows.len() - 1] == EMPTY_ROW {
            self.rows.pop();
        }
    }

    fn place(&mut self, rock: &Rock, top_x: isize, top_y: usize) {
        rock.bm.iter().enumerate().for_each(|(rock_y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, cell)| !cell_empty(**cell))
                .for_each(|(rock_x, cell)| {
                    self.put_cell(top_x + rock_x as isize, top_y + rock_y, *cell)
                });
        });
        self.shrink();
    }

    fn try_place(&self, rock: &Rock, top_x: isize, top_y: usize) -> Field {
        let mut rv = self.clone();
        rv.place(&rock.tentative(), top_x, top_y);
        rv
    }

    fn show(&self) {
        return;
        self.rows.iter().rev().for_each(|row| {
            row.iter().for_each(|cell| print!("{}", *cell as char));
            println!();
        });
        println!();
    }
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
