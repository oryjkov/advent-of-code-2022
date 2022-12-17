use std::fs;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), _);
    }
    #[test]
    fn test_part2() {}
}

fn rocks() -> Vec<Rock> {
    vec![
        Rock {
            bm: vec![vec![1, 1, 1, 1]],
        },
        Rock {
            bm: vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
        },
        Rock {
            bm: vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 1, 1]],
        },
        Rock {
            bm: vec![vec![1], vec![1], vec![1], vec![1]],
        },
        Rock {
            bm: vec![vec![1, 1], vec![1, 1]],
        },
    ]
}

fn solve_part1(f: &str) -> i32 {
    let jets = fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>()[0]
        .as_bytes();
    let rocks = rocks();
    let mut field = Field {
        rows: vec![[1, 1, 1, 1, 1, 1, 1, 1, 1]],
    };
    field.grow(4);
    let rock = &rocks[0];
    let top_x = 2;
    let mut top_y = 0;
    loop {
        field.try_place(&rocks[0], top_x, top_y).show();
        if field.check(rock, top_x, top_y + 1) {
            top_y += 1;
        } else {
            break;
        }
        println!();
    }

    field.place(&rocks[0], top_x, top_y);
    field.show();
    -1
}

fn solve_part2(f: &str) -> i32 {
    fs::read_to_string(f).unwrap().lines().count();
    -1
}

#[derive(Clone)]
struct Field {
    rows: Vec<[u8; 9]>,
}
#[derive(Clone, Debug)]
struct Rock {
    bm: Vec<Vec<u8>>,
}
impl Rock {
    fn tentative(&self) -> Self {
        let mut rv = self.clone();
        rv.bm.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|r| {
                if *r > 0 {
                    *r = 2
                }
            })
        });
        rv
    }
}

impl Field {
    fn height(&self) -> usize {
        self.rows.len() - 1
    }
    // Checks if placing a rock at x and with top at y is valid.
    fn check(&self, rock: &Rock, top_x: usize, top_y: usize) -> bool {
        rock.bm
            .iter()
            .enumerate()
            .try_fold(false, |_, (rock_y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, r)| **r > 0)
                    .try_fold(false, |_, (rock_x, _)| {
                        let x = rock_x + top_x;
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
    fn get_cell(&self, x: usize, y: usize) -> bool {
        let actual_y = self.rows.len() - y - 1;
        let actual_x = x + 1;
        if actual_y >= self.rows.len() {
            false
        } else if x >= self.rows[0].len() {
            true
        } else {
            self.rows[actual_y][actual_x] != 0
        }
    }
    fn put_cell(&mut self, x: usize, y: usize, r: u8) {
        let actual_y = self.rows.len() - y - 1;
        let actual_x = x + 1;
        self.rows[actual_y][actual_x] = r;
    }

    fn grow(&mut self, n: usize) {
        (0..n).for_each(|_| self.rows.push([1, 0, 0, 0, 0, 0, 0, 0, 1]));
    }

    fn place(&mut self, rock: &Rock, top_x: usize, top_y: usize) {
        rock.bm.iter().enumerate().for_each(|(rock_y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, r)| **r > 0)
                .for_each(|(rock_x, r)| {
                    self.put_cell(top_x + rock_x, top_y + rock_y, *r);
                });
        });
    }

    fn try_place(&self, rock: &Rock, top_x: usize, top_y: usize) -> Field {
        let mut rv = self.clone();
        rv.place(&rock.tentative(), top_x, top_y);
        rv
    }

    fn show(&self) {
        self.rows.iter().rev().for_each(|row| {
            row.iter().for_each(|r| {
                print!(
                    "{}",
                    if *r == 0 {
                        '.'
                    } else if *r == 2 {
                        '@'
                    } else {
                        'x'
                    }
                )
            });
            println!();
        })
    }
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
