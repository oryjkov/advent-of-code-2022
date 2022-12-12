use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        assert_eq!(31, solve_p1("test.txt"));
        assert_eq!(528, solve_p1("input.txt"));
    }
    #[test]
    fn test_solve2() {
        assert_eq!(29, solve_p2("test.txt"));
    }
}
#[derive(Clone, Copy)]
struct PathElement {
    cost: usize,
    dir: char,
}
struct Map {
    m: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    limit: (usize, usize),
}
impl Map {
    fn read(f: &str) -> Self {
        let mut map = Map {
            m: vec![],
            start: (0, 0),
            end: (0, 0),
            limit: (0, 0),
        };
        fs::read_to_string(f)
            .unwrap()
            .split('\n')
            .filter(|l| l.len() > 0)
            .enumerate()
            .map(|(row_num, l)| {
                let mut line = vec![];
                l.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(col_num, b)| {
                        line.push(match *b as char {
                            'S' => {
                                map.start = (row_num, col_num);
                                0
                            }
                            'E' => {
                                map.end = (row_num, col_num);
                                'z' as u8 - 'a' as u8
                            }
                            _ => b - 'a' as u8,
                        });
                    })
                    .count();
                map.m.push(line);
            })
            .count();
        map.limit = (map.m.len(), map.m[0].len());
        map
    }

    fn walk(&self) -> Vec<Vec<PathElement>> {
        fn check_bounds(
            (dr, dc): (i32, i32),
            start: (usize, usize),
            limit: (usize, usize),
        ) -> bool {
            let r = dr + start.0 as i32;
            let c = dc + start.1 as i32;
            r >= 0 && (r as usize) < limit.0 && c >= 0 && (c as usize) < limit.1
        }
        fn walk_helper(
            map: &Vec<Vec<u8>>,
            grid: &mut Vec<Vec<PathElement>>,
            start: (usize, usize),
            limit: (usize, usize),
        ) -> Vec<(usize, usize)> {
            fn char_to_dir(c: char) -> (i32, i32) {
                match c {
                    '<' => (0, -1),
                    '>' => (0, 1),
                    'v' => (-1, 0),
                    '^' => (1, 0),
                    _ => (0, 0),
                }
            }
            let mut next_rc = vec![];
            vec!['<', 'v', '>', '^']
                .iter()
                .filter(|c| check_bounds(char_to_dir(**c), start, limit))
                .map(|c| {
                    let (dr, dc) = char_to_dir(*c);
                    let row = (start.0 as i32 + dr) as usize;
                    let col = (start.1 as i32 + dc) as usize;
                    (row, col, *c)
                })
                .map(|(row, col, d)| {
                    //println!( "candidate: {:?} {:?}->{:?}", (row, col), map[start.0][start.1], map[row][col]);
                    let new_height = map[row][col];
                    let old_height = map[start.0][start.1];

                    if old_height as i32 - 1 <= new_height as i32 {
                        if grid[row][col].cost > grid[start.0][start.1].cost + 1 {
                            next_rc.push((row, col));
                            grid[row][col].cost = grid[start.0][start.1].cost + 1;
                            grid[row][col].dir = d;
                        }
                    }
                })
                .count();
            next_rc
        }

        let mut grid = vec![
            vec![
                PathElement {
                    cost: 10000,
                    dir: '.'
                };
                self.m[0].len()
            ];
            self.m.len()
        ];
        grid[self.end.0][self.end.1].cost = 0;
        grid[self.end.0][self.end.1].dir = 'E';
        let mut next_rc = vec![self.end];
        while next_rc.len() > 0 {
            let start = next_rc.pop().unwrap();
            //println!("start from: {:?}", start);
            next_rc.append(&mut walk_helper(&self.m, &mut grid, start, self.limit));
        }
        grid
    }
}

fn print_map(map: &Vec<Vec<u8>>) {
    map.iter()
        .map(|row| {
            row.iter()
                .map(|n| {
                    print!("{}", (n + 'a' as u8) as char);
                })
                .count();
            println!();
        })
        .count();
}

fn print_grid(grid: &Vec<Vec<PathElement>>) {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|n| {
                    print!("{}", n.dir);
                })
                .count();
            println!();
        })
        .count();
}

fn solve_p1(f: &str) -> i32 {
    let map = Map::read(f);
    //print_map(&map.m);
    //println!();
    let grid = map.walk();
    //print_grid(&grid);
    grid[map.start.0][map.start.1].cost as i32
}
fn solve_p2(f: &str) -> i32 {
    let map = Map::read(f);
    let grid = map.walk();
    //print_grid(&grid);
    let mut min = 10000;
    map.m
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, n)| {
                    if *n == 0 {
                        min = min.min(grid[r][c].cost);
                    }
                })
                .count();
        })
        .count();

    min as i32
}
fn main() {
    println!("part1: {}", solve_p1("input.txt"));
    println!("part2: {}", solve_p2("input.txt"));
}
