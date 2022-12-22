use std::{fs, str::from_utf8};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Void,
    Empty,
    Wall,
}
use Tile::*;
impl Tile {
    fn from_byte(b: u8) -> Tile {
        match b {
            b' ' => Void,
            b'.' => Empty,
            b'#' => Wall,
            _ => panic!("unknown tile character"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Edge(usize, usize);
struct Map {
    map: Vec<Vec<Tile>>,
    row_edges: Vec<Edge>,
    col_edges: Vec<Edge>,
}

impl Map {
    fn row_edges(&self, row: usize) -> Edge {
        self.row_edges[row]
    }
    fn col_edges(&self, row: usize) -> Edge {
        self.col_edges[row]
    }
    fn parse(inp: &[String]) -> Self {
        let num_rows = inp.len();
        let num_cols = inp.iter().map(|l| l.len()).max().unwrap();

        let mut m = Map {
            map: vec![vec![Void; num_cols]; num_rows],
            row_edges: vec![],
            col_edges: vec![],
        };
        inp.iter().enumerate().for_each(|(row_num, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .for_each(|(col_num, elem)| {
                    m.map[row_num][col_num] = Tile::from_byte(*elem);
                })
        });
        m.row_edges = m
            .map
            .iter()
            .map(|row| {
                let mut left_edge = 0;
                let mut right_edge = row.len();
                let mut i = 0;
                while let Some(t) = row.get(i) {
                    if *t != Void {
                        break;
                    }
                    i += 1;
                }
                left_edge = i;
                while let Some(t) = row.get(i) {
                    if *t == Void {
                        break;
                    }
                    i += 1;
                }
                right_edge = i;

                Edge(left_edge, right_edge)
            })
            .collect();

        for col in 0..num_cols {
            m.col_edges.push({
                let mut top_edge = 0;
                let mut bottom_edge = num_rows;
                let mut i = 0;
                while i < num_rows && m.map[i][col] == Void {
                    i += 1;
                }
                top_edge = i;
                while i < num_rows && m.map[i][col] != Void {
                    i += 1;
                }
                bottom_edge = i;

                Edge(top_edge, bottom_edge)
            });
        }

        m
    }
    fn top_left(&self) -> Position {
        Position {
            row: 0,
            col: self.row_edges(0).0,
            dir: 0,
        }
    }
    fn num_rows(&self) -> usize {
        self.row_edges.len()
    }
    fn num_cols(&self) -> usize {
        self.col_edges.len()
    }

    fn iter<'a, It>(&'a self, path: It) -> MapIter<'a, It> {
        MapIter {
            position: self.top_left(),
            map: &self,
            path_iter: path,
        }
    }
}

struct PathIter<'a> {
    pre_parsed: Option<Move>,
    i: core::slice::Iter<'a, u8>,
}
impl<'a> PathIter<'a> {
    fn from_bytes(b: &'a [u8]) -> Self {
        PathIter {
            pre_parsed: None,
            i: b.iter(),
        }
    }
}
impl<'a> Iterator for PathIter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        fn consume(m: Option<&u8>) -> Result<Move, Option<u8>> {
            match m {
                None => Err(None),
                Some(c) => {
                    if *c == b'L' {
                        Ok(RotateLeft)
                    } else if *c == b'R' {
                        Ok(RotateRight)
                    } else if *c == b'_' {
                        Ok(Sleep)
                    } else {
                        Err(Some(*c))
                    }
                }
            }
        }
        let rv;
        (rv, self.pre_parsed) = self.pre_parsed.as_ref().map_or_else(
            || {
                let mut buf = vec![];
                let z = loop {
                    let read = consume(self.i.next());
                    match read {
                        Ok(m) => break Some(m),
                        Err(maybe_u8) => match maybe_u8 {
                            None => break None,
                            Some(b) => {
                                buf.push(b);
                            }
                        },
                    }
                };
                if buf.len() == 0 {
                    (z, None)
                } else {
                    let num = from_utf8(&buf).unwrap().parse::<usize>().unwrap();
                    buf.clear();
                    (Some(Step(num)), z)
                }
            },
            |pre_parsed| (Some(pre_parsed.clone()), None),
        );
        rv
    }
}

enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
#[derive(PartialEq, Eq, Debug, Clone)]
enum Move {
    Sleep,
    RotateLeft,
    RotateRight,
    Step(usize),
}
use Move::*;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Position {
    row: usize,
    col: usize,
    dir: u8,
}
impl Position {
    fn new(row: usize, col: usize, dir: u8) -> Self {
        Position { row, col, dir }
    }
    fn to_answer(&self) -> usize {
        (self.row + 1) * 1000 + (self.col + 1) * 4 + self.dir as usize
    }
}

struct MapIter<'a, It> {
    position: Position,
    map: &'a Map,
    path_iter: It,
}

impl<'a, It> Iterator for MapIter<'a, It>
where
    It: Iterator<Item = Move>,
{
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.path_iter.next().map(|p| {
            match p {
                Sleep => (),
                RotateLeft => self.position.dir = (self.position.dir + 3) % 4,
                RotateRight => self.position.dir = (self.position.dir + 1) % 4,
                Step(num) => {
                    if self.position.dir % 2 == 0 {
                        // left or right move
                        let (row, mut col) = (self.position.row, self.position.col);
                        let edges = self.map.row_edges(row);
                        let row_len = edges.1 - edges.0;
                        let delta = if self.position.dir == 0 {
                            1
                        } else {
                            row_len - 1
                        };
                        for _ in 0..num {
                            let candidate_col = (col - edges.0 + delta) % row_len + edges.0;
                            if self.map.map[row][candidate_col] == Wall {
                                break;
                            }
                            col = candidate_col;
                        }
                        self.position.col = col;
                    } else {
                        // up or down move
                        let (mut row, col) = (self.position.row, self.position.col);
                        let edges = self.map.col_edges(col);
                        let col_len = edges.1 - edges.0;
                        let delta = if self.position.dir == 3 {
                            // up
                            col_len - 1
                        } else {
                            //down
                            1
                        };
                        //println!( "row: {}, col: {}, step: {}, edges: {:?}", row, col, delta, edges);
                        for _ in 0..num {
                            let candidate_row = (row - edges.0 + delta) % col_len + edges.0;
                            if self.map.map[candidate_row][col] == Wall {
                                break;
                            }
                            row = candidate_row;
                        }
                        self.position.row = row;
                    }
                }
            }

            self.position
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map() {
        let inp = vec![
            " ... ".to_string(),
            ".....".to_string(),
            ".... ".to_string(),
        ];
        let m = Map::parse(&inp);
        assert_eq!(m.row_edges(0), Edge(1, 4));
        assert_eq!(m.row_edges(1), Edge(0, 5));
        assert_eq!(m.row_edges(2), Edge(0, 4));
        assert_eq!(m.col_edges(0), Edge(1, 3));
        assert_eq!(m.col_edges(1), Edge(0, 3));
        assert_eq!(m.col_edges(2), Edge(0, 3));
        assert_eq!(m.col_edges(3), Edge(0, 3));
        assert_eq!(m.col_edges(4), Edge(1, 2));
        assert_eq!(m.num_rows(), 3);
        assert_eq!(m.num_cols(), 5);
    }
    #[test]
    fn test_walk() {
        let inp = vec![
            " .#. ".to_string(),
            ".... ".to_string(),
            ".... ".to_string(),
        ];
        let m = Map::parse(&inp);
        assert_eq!(
            m.top_left(),
            Position {
                row: 0,
                col: 1,
                dir: 0
            }
        );
        let pi = PathIter::from_bytes("_LR10R1L4".as_bytes());
        let mut walker = m.iter(pi);
        assert_eq!(walker.next().unwrap(), Position::new(0, 1, 0));
        assert_eq!(walker.next().unwrap(), Position::new(0, 1, 3));
        assert_eq!(walker.next().unwrap(), Position::new(0, 1, 0));
        assert_eq!(walker.next().unwrap(), Position::new(0, 1, 0));
        assert_eq!(walker.next().unwrap(), Position::new(0, 1, 1));
        assert_eq!(walker.next().unwrap(), Position::new(1, 1, 1));
        assert_eq!(walker.next().unwrap(), Position::new(1, 1, 0));
        assert_eq!(walker.next().unwrap(), Position::new(1, 1, 0));
        assert_eq!(walker.next(), None);
    }
    #[test]
    fn test_path_iter() {
        let mut pi = PathIter::from_bytes("_LR10R1".as_bytes());
        assert_eq!(pi.next().unwrap(), Sleep);
        assert_eq!(pi.next().unwrap(), RotateLeft);
        assert_eq!(pi.next().unwrap(), RotateRight);
        assert_eq!(pi.next().unwrap(), Step(10));
        assert_eq!(pi.next().unwrap(), RotateRight);
        assert_eq!(pi.next().unwrap(), Step(1));
        assert_eq!(pi.next(), None);
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 6032);
        assert_eq!(solve_part1("test.txt"), 164014);
    }
    #[test]
    fn test_part2() {}
}

fn solve_part1(f: &str) -> usize {
    let inp = fs::read_to_string(f).unwrap();
    let mut i = inp.split("\n\n");
    let maze = i
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let path = i.next().unwrap().lines().next().unwrap();
    let map = Map::parse(&maze);
    let walker = map.iter(PathIter::from_bytes(path.as_bytes()));
    /*
    walker.for_each(|pos| {
        println!("{:?}", pos);
    });
     */
    walker.last().unwrap().to_answer()
}

fn solve_part2(f: &str) -> i32 {
    fs::read_to_string(f).unwrap().lines().count();
    -1
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    //println!("part 2: {}", solve_part2("input.txt"));
}
