use std::{
    collections::VecDeque,
    fs,
    ops::{Shl, ShlAssign},
    time::{Duration, Instant},
};

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

#[derive(Clone, Copy)]
struct BitRock {
    bm: u32,
}
fn bit_rocks() -> Vec<(BitRock, usize)> {
    vec![
        ([0, 0, 0, 15 << 1], 1),
        ([0, 1 << 3, 7 << 2, 1 << 3], 3),
        ([0, 1 << 2, 1 << 2, 7 << 2], 3),
        ([1 << 4, 1 << 4, 1 << 4, 1 << 4], 4),
        ([0, 0, 3 << 3, 3 << 3], 2),
    ]
    .iter()
    .map(|(ba, height)| {
        let mut bm = 0u32;
        bm = bm + ba[0];
        bm = bm.overflowing_shl(8).0 + ba[1];
        bm = bm.overflowing_shl(8).0 + ba[2];
        bm = bm.overflowing_shl(8).0 + ba[3];

        (BitRock { bm }, *height)
    })
    .collect()
}

#[derive(Clone)]
struct BitField {
    rows: Vec<u8>,
    cleared: usize,
}
impl BitField {
    fn new() -> Self {
        BitField {
            rows: vec![0x7f],
            cleared: 0,
        }
    }
    fn height(&self) -> usize {
        self.rows.len() - 1 + self.cleared
    }
    // returns true if placement worked.
    fn show(&self) {
        for (n, row) in self.rows.iter().enumerate().rev() {
            if n == 0 {
                continue;
            }
            print!("{:4} {:3} |", n, *row);
            for s in (0..7).rev() {
                print!("{}", if *row & (1 << s) == 0 { '.' } else { '#' });
            }
            println!("|");
        }
        println!("{:4}     +-------+", -1);
    }
    fn shrink_bottom(&mut self) {
        let cleared = self.rows.len() / 2;
        self.rows = self.rows.split_off(cleared);
        self.cleared += cleared;
        return;

        let mut mask = 0u8;
        let mut i = self.rows.len() - 1;
        while mask != 0x7f {
            mask |= self.rows[i];
            if i == 0 {
                return;
            }
            i -= 1;
        }
        let can_clear = self.rows.len() - i;
        self.rows = self.rows.split_off(can_clear);
        self.cleared += can_clear;
        return;
    }
}

fn find_cycles(jets: &[u8], n: usize) -> Option<usize> {
    let rocks = bit_rocks();
    let mut field = BitField::new();
    let start = Instant::now();

    let mut shift_table = [0u32; 5 * 8];
    let mut rock_idx = 0;
    for (r, _) in bit_rocks() {
        for s in 0usize..=7 {
            let mut rock = r.bm.clone();
            for j in (0..3).rev() {
                let shift = s & (1 << j);
                let shifted = if shift == 0 {
                    rock << 1 & 0x7f7f7f7f
                } else {
                    rock >> 1 & 0x7f7f7f7f
                };

                if rock.count_ones() == shifted.count_ones() {
                    rock = shifted;
                }
            }
            shift_table[rock_idx * 8 + s] = rock;
        }
        rock_idx += 1;
    }
    let jl = jets.len();

    let mut pre_shifted = vec![0; jl];
    for i in 0..jl {
        let mut sm = 0;
        let mut jets_offset = i % jl;
        for _ in 0..3 {
            sm = sm << 1 | jets[jets_offset];
            jets_offset = (jets_offset + 1) % jl;
        }
        pre_shifted[i] = sm;
    }

    let mut jets_offset = 0;
    let lcm = rocks.len() * jets.len();
    let mut at_lcm = 0;
    let mut seq = vec![];
    for i in 0..n {
        let rock_num = i % 5;
        let rock_height = rocks[rock_num].1;
        let mut field_mask = 0u32;
        let mut depth = 0;

        let mut rock = shift_table[rock_num * 8 + pre_shifted[jets_offset] as usize];
        jets_offset = (jets_offset + 3) % jl;
        loop {
            let shift = pre_shifted[jets_offset] & 4;
            jets_offset = (jets_offset + 1) % jl;
            //println!("shift: '{}', before: {:8x}", shift as char, rock);
            let shifted = if shift == 0 {
                rock.rotate_left(1)
            } else {
                rock.rotate_right(1)
            };
            if (shifted & 0x80808080) == 0 && (shifted & field_mask) == 0 {
                rock = shifted;
            }
            field_mask =
                field_mask.overflowing_shl(8).0 + field.rows[field.rows.len() - 1 - depth] as u32;
            //println!("depth: {depth}, rock: {:8x}, mask: {:8x}", rock, field_mask);
            if rock & field_mask != 0 {
                break;
            }
            depth += 1;
        }

        let len = field.rows.len();

        for j in len - depth..len {
            //println!("{:2x} to row {}", (rock & 0xff) as u8, j);
            field.rows[j] |= (rock & 0xff) as u8;
            rock = rock.overflowing_shr(8).0;
        }

        for _ in len..len - depth + rock_height {
            field.rows.push((rock & 0xff) as u8);
            rock = rock.overflowing_shr(8).0;
        }

        if field.rows.len() > 1_000_000 {
            field.shrink_bottom();
        }
        if i % lcm == 0 && i > 0 {
            //println!("height diff at lcm: {}", field.height() - at_lcm);
            seq.push(field.height() - at_lcm);
            at_lcm = field.height();
        }
    }
    // find cycles
    let cycle_start = 1;
    for period in 1..seq.len() / 8 {
        //println!("try period {}", period);
        let mut ok = false;
        let mut j = cycle_start;
        while j + period < seq.len() {
            ok = true;
            if seq[j] != seq[period + j] {
                //println!( "failed at j={j},{}, {} != {}", period + j, seq[j], seq[period + j]);
                ok = false;
                break;
            }
            j += period;
        }
        if ok {
            return Some(period);
        }
    }
    None
}

// Finds a cycle and uses it to speed up search.
fn cycle_solve(f: &str, n: usize) -> i64 {
    let jets = fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>()[0]
        .as_bytes()
        .iter()
        .map(|b| if *b == b'>' { 1 } else { 0 })
        .collect::<Vec<u8>>();

    let rocks = bit_rocks();
    let mut field = BitField::new();

    let mut shift_table = [0u32; 5 * 8];
    let mut rock_idx = 0;
    for (r, _) in bit_rocks() {
        for s in 0usize..=7 {
            let mut rock = r.bm.clone();
            for j in (0..3).rev() {
                let shift = s & (1 << j);
                let shifted = if shift == 0 {
                    rock << 1 & 0x7f7f7f7f
                } else {
                    rock >> 1 & 0x7f7f7f7f
                };

                if rock.count_ones() == shifted.count_ones() {
                    rock = shifted;
                }
            }
            shift_table[rock_idx * 8 + s] = rock;
        }
        rock_idx += 1;
    }
    let jl = jets.len();

    let mut pre_shifted = vec![0; jl];
    for i in 0..jl {
        let mut sm = 0;
        let mut jets_offset = i % jl;
        for _ in 0..3 {
            sm = sm << 1 | jets[jets_offset];
            jets_offset = (jets_offset + 1) % jl;
        }
        pre_shifted[i] = sm;
    }

    let mut jets_offset = 0;
    let lcm = rocks.len() * jets.len();

    let start_from = 1;
    let cycle_length = find_cycles(&jets, 351*8*(jets.len()*rocks.len()).min(n)).unwrap_or(0);
    let mut height_at_start = 0;
    let mut cycle_height = 0;
    let mut i = 0;
    while i < n {
        let rock_num = i % 5;
        let rock_height = rocks[rock_num].1;
        let mut field_mask = 0u32;
        let mut depth = 0;

        let mut rock = shift_table[rock_num * 8 + pre_shifted[jets_offset] as usize];
        jets_offset = (jets_offset + 3) % jl;
        loop {
            let shift = pre_shifted[jets_offset] & 4;
            jets_offset = (jets_offset + 1) % jl;
            //println!("shift: '{}', before: {:8x}", shift as char, rock);
            let shifted = if shift == 0 {
                rock.rotate_left(1)
            } else {
                rock.rotate_right(1)
            };
            if (shifted & 0x80808080) == 0 && (shifted & field_mask) == 0 {
                rock = shifted;
            }
            field_mask =
                field_mask.overflowing_shl(8).0 + field.rows[field.rows.len() - 1 - depth] as u32;
            //println!("depth: {depth}, rock: {:8x}, mask: {:8x}", rock, field_mask);
            if rock & field_mask != 0 {
                break;
            }
            depth += 1;
        }

        let len = field.rows.len();

        for j in len - depth..len {
            //println!("{:2x} to row {}", (rock & 0xff) as u8, j);
            field.rows[j] |= (rock & 0xff) as u8;
            rock = rock.overflowing_shr(8).0;
        }

        //field.rows.extend(vec![0; rock_height - depth]);
        for _ in len..len - depth + rock_height {
            field.rows.push((rock & 0xff) as u8);
            rock = rock.overflowing_shr(8).0;
        }

        if i % lcm == 0 && cycle_length > 0 {
            if i / lcm == start_from {
                height_at_start = field.height();
                //println!( "cycle starts at {}, height: {}", start_from, height_at_start);
            } else if i / lcm == cycle_length + start_from {
                cycle_height = field.height() - height_at_start;
                //println!( "cycle length: {}, cycle growth = {}", cycle_length, cycle_height);
                loop {
                    if i + cycle_length * lcm < n {
                        i += cycle_length * lcm;
                        field.cleared += cycle_height;
                    } else {
                        break;
                    }
                }
            }
            //i += 1;
        }
        i += 1;
    }
    field.height() as i64
}

fn solve(f: &str, n: usize) -> (i64, usize) {
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
    let mut jets_idx = 0;
    for (rock_num, rock) in rocks.iter().cycle().enumerate() {
        if rock_num >= n {
            break;
        }
        field.grow(rock.height() + 3);
        let mut top_x = 2;
        let mut top_y = 0;
        for offset in &mut jets_iter {
            jets_idx += 1;
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
            //field.shrink_bottom();
        }
        if rock_num % 10_000_000 == 1_000_000 {
            let mrps = (rock_num as f64) / start.elapsed().as_secs_f64() / 1e6;
            let will_take = Duration::from_secs_f64(((n - rock_num) as f64 / mrps) / 1e6);
            //println!( "{}, Mrps: {:.2}, will take: {:?}", rock_num, mrps, will_take);
        }
    }
    field.show();
    println!("jets used: {jets_idx}");
    (field.height() as i64, jets_idx)
}

fn solve_part1(f: &str) -> i64 {
    cycle_solve(f, NUM_ROCKS)
}
fn solve_part2(f: &str) -> i64 {
    cycle_solve(f, N2)
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
        //return;
        self.rows.iter().enumerate().rev().for_each(|(idx, row)| {
            print!("{:4} ", idx);
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
