use std::{collections::HashMap, fs, str::from_utf8};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Void,
    Empty,
    Wall,
    Dir(u8),
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
    fn to_byte(&self) -> u8 {
        match self {
            Void => b'!',
            Empty => b'.',
            Wall => b'#',
            Dir(b) => *b,
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
    fn step(&self, pos: Position) -> Position {
        if pos.dir % 2 == 0 {
            // left or right move
            let edges = self.row_edges(pos.row);
            let row_len = edges.1 - edges.0;
            let delta = if pos.dir == 0 { 1 } else { row_len - 1 };

            let candidate_col = (pos.col - edges.0 + delta) % row_len + edges.0;
            if self.map[pos.row][candidate_col] != Wall {
                Position::new(pos.row, candidate_col, pos.dir)
            } else {
                pos
            }
        } else {
            // up or down move
            let edges = self.col_edges(pos.col);
            let col_len = edges.1 - edges.0;
            let delta = if pos.dir == 3 {
                // up
                col_len - 1
            } else {
                //down
                1
            };
            //println!( "row: {}, col: {}, step: {}, edges: {:?}", row, col, delta, edges);
            let candidate_row = (pos.row - edges.0 + delta) % col_len + edges.0;
            if self.map[candidate_row][pos.col] != Wall {
                Position::new(candidate_row, pos.col, pos.dir)
            } else {
                pos
            }
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

struct FlatPathIter<'a> {
    steps_left: usize,
    i: PathIter<'a>,
}
impl<'a> FlatPathIter<'a> {
    fn from_path_iter(path_iter: PathIter<'a>) -> Self {
        FlatPathIter {
            steps_left: 0,
            i: path_iter,
        }
    }
}
impl<'a> Iterator for FlatPathIter<'a> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_left > 0 {
            self.steps_left -= 1;
            Some(Step(1))
        } else {
            self.i.next().map(|m| {
                if let Step(n) = m {
                    self.steps_left = n - 1;
                    Step(1)
                } else {
                    m
                }
            })
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
impl Direction {
    fn to_u8(&self) -> u8 {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
    fn u8_to_printable(d: u8) -> u8 {
        match d {
            0 => b'>',
            1 => b'v',
            2 => b'<',
            3 => b'^',
            _ => panic!("oopsie"),
        }
    }
}
fn flip_dir_u8(d: u8) -> u8 {
    (d + 2) % 4
}

use Direction::*;
#[derive(PartialEq, Eq, Debug, Clone)]
enum Move {
    Sleep,
    RotateLeft,
    RotateRight,
    Step(usize),
}
use Move::*;
type EdgeFn = fn(&CubePosition, usize, isize) -> CubePosition;
type Wraps = HashMap<(usize, u8), (usize, u8, EdgeFn)>;
type Face = Vec<Vec<Tile>>;
type Blocks = Vec<(usize, usize, usize)>;
#[derive(Clone)]
struct CubeMap {
    faces: Vec<Face>,
    wraps: Wraps,
    edge_len: usize,
    blocks: Blocks,
}

fn straight_edge(p: &CubePosition, new_face: usize, edge_len: isize) -> CubePosition {
    let new_dir = p.dir;
    let new_row = if p.row == edge_len as isize {
        0
    } else if p.row == -1 {
        edge_len as isize - 1
    } else {
        p.row
    };

    let new_col = if p.col == edge_len as isize {
        0
    } else if p.col == -1 {
        edge_len as isize - 1
    } else {
        p.col
    };
    CubePosition::new(new_row, new_col, new_face, new_dir)
}

fn flip_edge(p: &CubePosition, new_face: usize, edge_len: isize) -> CubePosition {
    let new_dir = (p.dir + 2) % 4;
    let new_col = if p.col == edge_len as isize {
        edge_len - 1
    } else if p.col == -1 {
        0
    } else {
        (edge_len - p.col) % edge_len
    };

    let new_row = if p.row == edge_len as isize {
        panic!("unexpected flip")
    } else if p.row == -1 {
        panic!("unexpected flip")
    } else {
        (edge_len - 1 - p.row) % edge_len
    };
    CubePosition::new(new_row, new_col, new_face, new_dir)
}
fn rot_left_edge(p: &CubePosition, new_face: usize, edge_len: isize) -> CubePosition {
    let new_dir = (p.dir + 3) % 4;
    let new_row = if p.col == edge_len as isize {
        edge_len - 1
    } else if p.col == -1 {
        0
    } else {
        panic!("unexpected rot left")
    };

    let new_col = if p.row == edge_len as isize {
        panic!("unexpected rot left")
    } else if p.row == -1 {
        panic!("unexpected rot left")
    } else {
        p.row
    };
    CubePosition::new(new_row, new_col, new_face, new_dir)
}

fn rot_right_edge(p: &CubePosition, new_face: usize, edge_len: isize) -> CubePosition {
    let new_dir = (p.dir + 1) % 4;
    let new_col = if p.row == edge_len as isize {
        edge_len - 1
    } else if p.row == -1 {
        0
    } else {
        panic!("unexpected rot right {}", p.row)
    };

    let new_row = if p.col == edge_len as isize {
        panic!("unexpected column flip")
    } else if p.col == -1 {
        panic!("unexpected column flip")
    } else {
        p.col
    };
    CubePosition::new(new_row, new_col, new_face, new_dir)
}

fn rotate<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut tmp = transpose(v);
    tmp.iter_mut().for_each(|row| row.reverse());
    tmp
}
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

impl CubeMap {
    fn print(&self) {
        let mut buf = vec![vec![b' '; self.edge_len * 4]; self.edge_len * 4];
        let blocks = vec![
            (1000, 1000, 0),
            (0, 1, 0),
            (0, 2, 0),
            (1, 1, 0),
            (2, 0, 0),
            (2, 1, 0),
            (3, 0, 0),
        ];

        for (idx, block) in blocks.iter().enumerate().skip(1) {
            let r_off = block.0 * self.edge_len;
            let c_off = block.1 * self.edge_len;
            for r in 0..self.edge_len {
                for c in 0..self.edge_len {
                    buf[r_off + r][c_off + c] = self.faces[idx][r][c].to_byte();
                }
            }
        }
        buf.iter().for_each(|row| {
            println!("{}", from_utf8(row).unwrap());
        })
    }
    fn parse(inp: &[String], blocks: &Blocks, edge_len: usize) -> Self {
        let max_width = inp.iter().map(|l| l.len()).max().unwrap();

        let inp = inp
            .iter()
            .map(|l| {
                let padding = vec![b' '; max_width - l.len()];
                let mut rv = l.as_bytes().to_vec();
                rv.extend(padding.iter().copied());
                rv
            })
            .collect::<Vec<Vec<u8>>>();

        //let edge_len = inp[0].len() / 3;
        //assert_eq!(edge_len * 3, inp[0].len());
        //assert_eq!(max_width, edge_len * 3);
        //assert_eq!(inp.len(), edge_len * 4);
        let mut wraps = HashMap::new();
        [
            (
                (1usize, Up),
                (
                    6usize,
                    Right,
                    rot_right_edge as EdgeFn,
                    rot_left_edge as EdgeFn,
                ),
            ),
            ((5, Down), (6, Left, rot_right_edge, rot_left_edge)),
            ((1, Left), (4, Right, flip_edge, flip_edge)),
            ((1, Down), (3, Down, straight_edge, straight_edge)),
            ((1, Right), (2, Right, straight_edge, straight_edge)),
            ((5, Left), (4, Left, straight_edge, straight_edge)),
            ((5, Right), (2, Left, flip_edge, flip_edge)),
            ((5, Up), (3, Up, straight_edge, straight_edge)),
            ((2, Up), (6, Up, straight_edge, straight_edge)),
            ((6, Up), (4, Up, straight_edge, straight_edge)),
            ((4, Up), (3, Right, rot_right_edge, rot_left_edge)),
            ((2, Down), (3, Left, rot_right_edge, rot_left_edge)),
        ]
        .iter()
        .for_each(|x| {
            let ((face1, dir1), (face2, dir2, fwd_edge, rev_edge)) = x;
            wraps.insert((*face1, dir1.to_u8()), (*face2, dir2.to_u8(), *fwd_edge));

            wraps.insert(
                (*face2, flip_dir_u8(dir2.to_u8())),
                (*face1, flip_dir_u8(dir1.to_u8()), *rev_edge),
            );
        });
        let faces = blocks
            .iter()
            .map(|(block_row, block_col, rots)| {
                if *block_row == 1000 {
                    return vec![];
                }
                let mut rv = vec![];
                println!("block {} {} {}", block_row, block_col, rots);
                for row in block_row * edge_len..(block_row + 1) * edge_len {
                    //rv.push(inp[(block_col*edge_len)..((block_col+1)*edge_len)])
                    rv.push(
                        inp[row][(block_col * edge_len)..((block_col + 1) * edge_len)]
                            .iter()
                            .map(|b| {
                                let rv = Tile::from_byte(*b);
                                assert_ne!(rv, Void);
                                rv
                            })
                            .collect(),
                    );
                }
                for _ in 0..*rots {
                    rv = rotate(rv);
                }
                rv
            })
            .collect::<Vec<Vec<Vec<Tile>>>>();

        CubeMap {
            faces,
            wraps,
            edge_len,
            blocks: blocks.clone(),
        }
    }

    fn step(&self, pos: CubePosition) -> CubePosition {
        let new_pos = match pos.dir {
            3 => {
                // Up
                let new_pos = CubePosition::new(pos.row - 1, pos.col, pos.face, pos.dir);
                if new_pos.row < 0 {
                    let (new_face, new_dir, edge_fn) =
                        self.wraps.get(&(pos.face, pos.dir)).unwrap();
                    let candidate_pos = edge_fn(&new_pos, *new_face, self.edge_len as isize);
                    assert_eq!(*new_dir, candidate_pos.dir);
                    candidate_pos
                } else {
                    new_pos
                }
            }
            1 => {
                // Down
                let new_pos = CubePosition::new(pos.row + 1, pos.col, pos.face, pos.dir);
                if new_pos.row >= self.edge_len as isize {
                    let (new_face, new_dir, edge_fn) =
                        self.wraps.get(&(pos.face, pos.dir)).unwrap();
                    let candidate_pos = edge_fn(&new_pos, *new_face, self.edge_len as isize);
                    assert_eq!(*new_dir, candidate_pos.dir);
                    candidate_pos
                } else {
                    new_pos
                }
            }
            2 => {
                // Left
                let new_pos = CubePosition::new(pos.row, pos.col - 1, pos.face, pos.dir);
                if new_pos.col < 0 {
                    let (new_face, new_dir, edge_fn) =
                        self.wraps.get(&(pos.face, pos.dir)).unwrap();
                    let candidate_pos = edge_fn(&new_pos, *new_face, self.edge_len as isize);
                    assert_eq!(*new_dir, candidate_pos.dir);
                    candidate_pos
                } else {
                    new_pos
                }
            }
            0 => {
                // Right
                let new_pos = CubePosition::new(pos.row, pos.col + 1, pos.face, pos.dir);
                if new_pos.col >= self.edge_len as isize {
                    let (new_face, new_dir, edge_fn) =
                        self.wraps.get(&(pos.face, pos.dir)).unwrap();
                    let candidate_pos = edge_fn(&new_pos, *new_face, self.edge_len as isize);
                    assert_eq!(*new_dir, candidate_pos.dir);
                    candidate_pos
                } else {
                    new_pos
                }
            }
            _ => panic!(),
        };
        if self.at(new_pos) == Wall {
            pos
        } else {
            new_pos
        }
    }
    fn at_mut(&mut self, p: CubePosition) -> &mut Tile {
        &mut self.faces[p.face][p.row as usize][p.col as usize]
    }
    fn at(&self, p: CubePosition) -> Tile {
        self.faces[p.face][p.row as usize][p.col as usize]
    }
    fn iter<'a, It>(&'a self, path: It) -> CubeMapIter<'a, It> {
        CubeMapIter {
            position: CubePosition::new(0, 0, 1, 0),
            map: &self,
            path_iter: path,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct CubePosition {
    row: isize,
    col: isize,
    face: usize,
    dir: u8,
}
impl CubePosition {
    fn new(row: isize, col: isize, face: usize, dir: u8) -> Self {
        CubePosition {
            row,
            col,
            face,
            dir,
        }
    }
    fn rotate_left(&self) -> Self {
        CubePosition::new(self.row, self.col, self.face, (self.dir + 3) % 4)
    }
    fn rotate_right(&self) -> Self {
        CubePosition::new(self.row, self.col, self.face, (self.dir + 1) % 4)
    }
    fn to_answer(&self, blocks: &Blocks, edge_len: isize) -> isize {
        println!("{:?}", self);
        ((blocks[self.face].0 as isize) * edge_len + self.row + 1) * 1000
            + ((blocks[self.face].1 as isize) * edge_len + self.col + 1) * 4
            + self.dir as isize
    }
}

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
    fn rotate_left(&self) -> Self {
        Position::new(self.row, self.col, (self.dir + 3) % 4)
    }
    fn rotate_right(&self) -> Self {
        Position::new(self.row, self.col, (self.dir + 1) % 4)
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
            self.position = match p {
                Sleep => self.position,
                RotateLeft => self.position.rotate_left(),
                RotateRight => self.position.rotate_right(),
                Step(num) => {
                    let mut new_pos = self.position;
                    for _ in 0..num {
                        new_pos = self.map.step(new_pos);
                    }
                    new_pos
                }
            };

            self.position
        })
    }
}

struct CubeMapIter<'a, It> {
    position: CubePosition,
    map: &'a CubeMap,
    path_iter: It,
}

impl<'a, It> Iterator for CubeMapIter<'a, It>
where
    It: Iterator<Item = Move>,
{
    type Item = CubePosition;

    fn next(&mut self) -> Option<Self::Item> {
        self.path_iter.next().map(|p| {
            self.position = match p {
                Sleep => self.position,
                RotateLeft => self.position.rotate_left(),
                RotateRight => self.position.rotate_right(),
                Step(num) => {
                    let mut new_pos = self.position;
                    for _ in 0..num {
                        new_pos = self.map.step(new_pos);
                    }
                    new_pos
                }
            };

            self.position
        })
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_cube_parse() {
        let inp = vec![
            "    ........".to_string(),
            "    ........".to_string(),
            "    ........".to_string(),
            "    ........".to_string(),
            "    ....".to_string(),
            "    ....".to_string(),
            "    ....".to_string(),
            "    ....".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "........".to_string(),
            "....".to_string(),
            "....".to_string(),
            "....".to_string(),
            "....".to_string(),
        ];
        let blocks = vec![
            (1000, 1000, 0),
            (0, 1, 0),
            (0, 2, 0),
            (1, 1, 0),
            (2, 0, 0),
            (2, 1, 0),
            (3, 0, 0),
        ];
        let m = CubeMap::parse(&inp, &blocks, 4);
        let p = CubePosition::new(0, 0, 1, Up.to_u8());
        assert_eq!(m.edge_len, 4);
        assert_eq!(m.step(p), CubePosition::new(0, 0, 6, Right.to_u8()));
        assert_eq!(
            m.step(CubePosition::new(3, 2, 5, Down.to_u8())),
            CubePosition::new(2, 3, 6, Left.to_u8())
        );
        assert_eq!(
            m.step(CubePosition::new(3, 0, 3, Left.to_u8())),
            CubePosition::new(0, 3, 4, Down.to_u8())
        );
        assert_eq!(
            m.step(CubePosition::new(1, 3, 2, Right.to_u8())),
            CubePosition::new(2, 3, 5, Left.to_u8())
        );

        for face in 1..=6 {
            for dir in 0..3 {
                for row in 0..4 {
                    for col in 0..4 {
                        let p0 = CubePosition::new(row, col, face, dir);
                        let mut p = p0.clone();
                        for i in 0..4 * 4 {
                            p = m.step(p);
                            if i != 4 * 4 - 1 {
                                assert_ne!(p0, p);
                            }
                        }
                        assert_eq!(p0, p);
                    }
                }
            }
        }
    }

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
        assert_eq!(solve_part1("input.txt"), 164014);
    }
    #[test]
    fn test_rotate() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let v2 = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        assert_eq!(rotate(v), v2);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("input.txt"), 47525);

    }
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

fn solve_part2(f: &str) -> isize {
    let inp = fs::read_to_string(f).unwrap();
    let mut i = inp.split("\n\n");
    let maze = i
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let path = i.next().unwrap().lines().next().unwrap();
    let blocks = vec![
        (1000, 1000, 0),
        (0, 1, 0),
        (0, 2, 0),
        (1, 1, 0),
        (2, 0, 0),
        (2, 1, 0),
        (3, 0, 0),
    ];
    let map = CubeMap::parse(&maze, &blocks, 50);
    let walker = map.iter(PathIter::from_bytes(path.as_bytes()));
    walker
        .last()
        .unwrap()
        .to_answer(&map.blocks, map.edge_len as isize)
    //fs::read_to_string(f).unwrap().lines().count();
}

fn solve_part2_test() -> isize {
    let inp = fs::read_to_string("test.txt").unwrap();
    let mut i = inp.split("\n\n");
    let maze = i
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let path = i.next().unwrap().lines().next().unwrap();
    let blocks = vec![
        (1000, 1000, 0),
        (0, 2, 0), // 1
        (2, 3, 2), // 2
        (1, 2, 0), // 3
        (1, 1, 3), // 4
        (2, 2, 0), // 5
        (1, 0, 3), // 6
    ];
    let map = CubeMap::parse(&maze, &blocks, 4);
    let mut w = map.clone();
    //map.print();
    let walker = map.iter(FlatPathIter::from_path_iter(PathIter::from_bytes(
        path.as_bytes(),
    )));
    walker.take(500).for_each(|pos| {
        *w.at_mut(pos) = Dir(Direction::u8_to_printable(pos.dir)); //Dir(pos.dir)
    });
    w.print();
    //fs::read_to_string(f).unwrap().lines().count();
    -1
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
    println!("part 2: {}", solve_part2_test());
}
