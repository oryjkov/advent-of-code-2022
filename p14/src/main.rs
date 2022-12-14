use std::fs;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 24);
    }
    #[test]
    fn test_part2() {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Air,
    Block,
    Sand,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Coord(usize, usize);

struct Reservoir {
    map: Vec<Vec<Tile>>,
    start: Coord,
    left_col: usize,
    height: usize,
    width: usize,
}
impl Reservoir {
    fn new() -> Self {
        Reservoir {
            map: vec![vec![Tile::Air; 0]; 0],
            start: Coord(0, 500),
            left_col: 1000,
            width: 0,
            height: 0,
        }
    }

    fn inside(&self, pos: Coord) -> bool {
        if pos.1 >= self.left_col + self.width {
            return false;
        }
        if pos.1 < self.left_col {
            return false;
        }
        if pos.0 >= self.height {
            return false;
        }
        true
    }
    // Outcomes:
    //  - found a new empty position on the map,
    //  - fell off screen,
    //  - this is the end position
    fn next_pos(&self, pos: Coord) -> Option<Coord> {
        for (dr, dc) in [(1, 0), (1, -1), (1, 1)].iter() {
            let candidate = Coord((pos.0 as i32 + *dr) as usize, (pos.1 as i32 + *dc) as usize);
            if !self.inside(candidate) {
                // off screen
                return None;
            }
            if self.at(candidate) == Tile::Air {
                // found the next position
                return Some(candidate);
            }
        }
        // stopped
        Some(pos)
    }
    // returns false when sand falls off screen
    fn drop_sand(&mut self) -> bool {
        let mut pos = self.start;
        loop {
            if let Some(new_pos) = self.next_pos(pos) {
                if pos == new_pos {
                    // Found the resting place.
                    *self.at_mut(pos) = Tile::Sand;
                    return true;
                }
                // keep going
                pos = new_pos;
            } else {
                // fell off screen
                return false;
            }
        }
    }
    fn grow_down(&mut self, max_row: usize) {
        //println!("grow down from {} to {}", self.height, max_row + 1);
        if self.height < max_row + 1 {
            self.map.append(&mut vec![
                vec![Tile::Air; self.width];
                max_row + 1 - self.height
            ]);
            self.height = max_row + 1;
        }
    }
    fn grow_side(&mut self, min_col: usize, max_col: usize) {
        if self.left_col == 1000 {
            self.left_col = min_col;
        }
        if min_col < self.left_col {
            //println!("grow left by: {}", self.left_col - min_col);
            (&mut self.map).into_iter().for_each(|row| {
                for _ in 0..(self.left_col - min_col) {
                    row.insert(0, Tile::Air);
                }
            });
            self.width += self.left_col - min_col;
            self.left_col = min_col;
        }

        if max_col + 1 - self.left_col > self.width {
            //println!( "grow right by: {}", max_col + 1 - self.left_col - self.width);
            let by = max_col + 1 - self.left_col - self.width;
            (&mut self.map).into_iter().for_each(|row| {
                row.append(&mut vec![Tile::Air; by]);
            });
            self.width += by;
        }
    }
    fn at(&self, coord: Coord) -> Tile {
        //println!("at {:?}", coord);
        self.map[coord.0][coord.1 - self.left_col]
    }
    fn at_mut(&mut self, coord: Coord) -> &mut Tile {
        //println!("at {:?}", coord);
        &mut self.map[coord.0][coord.1 - self.left_col]
    }
    fn insert_block(&mut self, from: Coord, to: Coord) {
        //println!("insert from {:?} to {:?}", from, to);
        let min_col = from.1.min(to.1);
        let max_col = from.1.max(to.1);
        let max_row = from.0.max(to.0);
        self.grow_side(min_col, max_col);
        self.grow_down(max_row);
        if from.0 == to.0 {
            // horizontal block
            let min = from.1.min(to.1);
            let max = from.1.max(to.1);
            for i in min..=max {
                *self.at_mut(Coord(from.0, i)) = Tile::Block;
            }
        } else if from.1 == to.1 {
            // vertical block
            let min = from.0.min(to.0);
            let max = from.0.max(to.0);
            for i in min..=max {
                *self.at_mut(Coord(i, from.1)) = Tile::Block;
            }
        }
    }
    fn print(&self) {
        //println!("width: {}, height: {}", self.width, self.height);
        //println!("width: {}, height: {}", self.map[0].len(), self.map.len());
        //println!("left_col: {}", self.left_col);
        //return;
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| {
                        let c = match t {
                            Tile::Air => '.',
                            Tile::Block => '#',
                            Tile::Sand => 'o',
                        };
                        print!("{c}");
                    })
                    .count();
                println!();
            })
            .count();
    }
}

fn read_map(f: &str) -> Reservoir {
    let mut r = Reservoir::new();
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .map(|l| {
            let v: Vec<Coord> = l
                .split(" -> ")
                .map(|p| {
                    let c = p
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    Coord(c[1], c[0])
                })
                .collect();
            v.as_slice()
                .windows(2)
                .map(|c| r.insert_block(c[1], c[0]))
                .count();
        })
        .count();
    r
}

fn solve_part1(f: &str) -> i32 {
    let mut r = read_map(f);
    r.print();
    let mut count = 0;
    while r.drop_sand() {
        count += 1;
        //r.print();
        //println!("=================");
    }
    count
}

fn solve_part2(f: &str) -> i32 {
    fs::read_to_string(f).unwrap().lines().count();
    -1
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
