use std::{fs, num, str::from_utf8};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_map() {
        let inp = vec![
            ".....".as_bytes(),
            "..##.".as_bytes(),
            "..#..".as_bytes(),
            ".....".as_bytes(),
            "..##.".as_bytes(),
            ".....".as_bytes(),
        ];
        let mut m = Map::parse(&inp);
        let [tl, br] = m.find_boundaries();
        assert_eq!(br.0 - tl.0 + 1, 4);
        assert_eq!(br.1 - tl.1 + 1, 2);

        assert_eq!(m.round(), 3);
        assert_eq!(m.round(), 5);
        assert_eq!(m.round(), 3);
        assert_eq!(m.round(), 0);
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 110);
        assert_eq!(solve_part1("input.txt"), 3800);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("test.txt"), 20);
        assert_eq!(solve_part2("input.txt"), 916);
    }
}

type Field = Vec<Vec<Tile>>;
#[derive(Clone)]
struct Elf {
    id: usize,
    move_to: Option<(usize, usize)>,
}
#[derive(Clone)]
struct Tile {
    elf: Option<Elf>,
    props: Vec<Proposal>,
}
impl Tile {
    fn new() -> Self {
        Tile {
            elf: None,
            props: vec![],
        }
    }
    fn to_u8(&self) -> u8 {
        match self.elf {
            None => b'.',
            Some(_) => b'#',
        }
    }
    fn props_to_u8(&self) -> u8 {
        format!("{}", self.props.len()).as_bytes()[0]
    }
}
#[derive(Clone)]
struct Proposal {
    elf_id: usize,
    from_row: usize,
    from_col: usize,
}
#[derive(Clone, Copy, Debug)]
enum Dir {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
}
impl Dir {
    fn to_delta(&self) -> (isize, isize) {
        use Dir::*;
        match self {
            N => (-1, 0),
            S => (1, 0),
            W => (0, -1),
            E => (0, 1),
            NE => (-1, 1),
            NW => (-1, -1),
            SE => (1, 1),
            SW => (1, -1),
            _ => panic!("wrong to"),
        }
    }
    fn apply_to_rc(&self, rc: (usize, usize)) -> (usize, usize) {
        let (dr, dc) = self.to_delta();
        ((rc.0 as isize + dr) as usize, (rc.1 as isize + dc) as usize)
    }
    fn checks(&self) -> [Dir; 3] {
        use Dir::*;
        match self {
            N => [N, NE, NW],
            S => [S, SE, SW],
            W => [W, NW, SW],
            E => [E, NE, SE],
            _ => panic!("wrong to"),
        }
    }
}

struct Map {
    map: Field,
    proposal_directions: Vec<Dir>,
    next_dir: usize,
    height: usize,
    width: usize,
}
impl Map {
    fn row_empty(&self, r: usize) -> bool {
        for c in 0..self.width {
            if self.map[r][c].elf.is_some() {
                return false;
            }
        }
        return true;
    }
    fn col_empty(&self, c: usize) -> bool {
        for r in 0..self.height {
            if self.map[r][c].elf.is_some() {
                return false;
            }
        }
        return true;
    }
    fn find_boundaries(&self) -> [(usize, usize); 2] {
        let mut r = 0;
        while r < self.map.len() && self.row_empty(r) {
            r += 1;
        }
        let r0 = r;
        r = self.height - 1;
        while r > r0 && self.row_empty(r) {
            r -= 1;
        }
        let r1 = r;

        let mut c = 0;
        while c < self.map.len() && self.col_empty(c) {
            c += 1;
        }
        let c0 = c;
        c = self.width - 1;
        while c > r0 && self.col_empty(c) {
            c -= 1;
        }
        let c1 = c;

        [(r0, c0), (r1, c1)]
    }
    fn empty_tiles(&self) -> usize {
        let [tl, br] = self.find_boundaries();
        self.map[tl.0..=br.0]
            .iter()
            .map(|row| row[tl.1..=br.1].iter().filter(|t| t.elf.is_none()).count())
            .sum()
    }
    fn print(&self) {
        let [tl, br] = self.find_boundaries();
        for r in tl.0 - 1..=br.0 + 1 {
            let s: Vec<u8> = self.map[r][tl.1 - 1..=br.1 + 1]
                .iter()
                .map(|t| t.to_u8())
                .collect();
            print!("{}  |  ", from_utf8(&s).unwrap());
            let s: Vec<u8> = self.map[r][tl.1 - 1..=br.1 + 1]
                .iter()
                .map(|t| t.props_to_u8())
                .collect();
            println!("{}", from_utf8(&s).unwrap());
        }
    }
    fn parse(inp: &[&[u8]]) -> Self {
        let width = inp[0].len();
        let height = inp.len();

        let target_width = width * 3;
        let target_height = height * 3;

        let mut elf_id = 0;

        let mut map = vec![vec![Tile::new(); target_width]; height];
        map.extend(inp.iter().map(|row| {
            let mut rv = vec![Tile::new(); width];
            rv.extend(row.iter().map(|c| {
                let e = match c {
                    b'.' => None,
                    b'#' => {
                        elf_id += 1;
                        Some(Elf {
                            id: elf_id,
                            move_to: None,
                        })
                    }
                    _ => panic!("wrong char"),
                };
                Tile {
                    elf: e,
                    props: vec![],
                }
            }));
            rv.append(&mut vec![Tile::new(); width]);
            rv
        }));
        map.append(&mut vec![vec![Tile::new(); target_width]; height]);

        let proposal_directions = vec![Dir::N, Dir::S, Dir::W, Dir::E];
        Map {
            map,
            proposal_directions,
            next_dir: 0,
            height: target_height,
            width: target_width,
        }
    }
    fn step1(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.map[row][col].elf.is_none() {
                    continue;
                }

                // whether this elf has any neighbours at all.
                let mut seen_elves = false;

                let elf_id = self.map[row][col].elf.as_ref().unwrap().id;
                let dir_id = self.next_dir;

                // Try the 4 directions in order starting from dir_id. If one
                // of the looks good, then propose it.
                let mut proposed_dir = None;
                for dp in 0..4 {
                    let candidate_dir = self.proposal_directions[(dir_id + dp) % 4];

                    // Number of elves in the given direction (3 cells per direction)
                    let mut num_elves = 0;
                    for dir_to_try in candidate_dir.checks() {
                        let rc = dir_to_try.apply_to_rc((row, col));
                        if self.map[rc.0][rc.1].elf.is_some() {
                            num_elves += 1;
                            seen_elves = true;
                        }
                    }
                    if num_elves == 0 && proposed_dir.is_none() {
                        proposed_dir = Some(candidate_dir);
                    }
                }
                //println!( "elf at ({}, {}), seen elves: {seen_elves}, prop: {:?}, {}", row, col, proposed_dir, self.next_dir);
                // No elves around => no move.
                if !seen_elves {
                    continue;
                }
                // Submit the proposal into the destination cell.
                if let Some(dir) = proposed_dir {
                    let rc = dir.apply_to_rc((row, col));
                    let props = &mut self.map[rc.0][rc.1].props;
                    props.push(Proposal {
                        elf_id,
                        from_row: row,
                        from_col: col,
                    });
                }
            }
        }
        // This step starts from the next direction on the next round.
        self.next_dir = (self.next_dir + 1) % 4;
    }

    fn step2(&mut self) -> usize {
        let mut moves = 0;
        // Go through proposals and act whenever there is only 1 proposal in a cell.
        for row in 1..self.map.len() - 1 {
            for col in 1..self.map[row].len() - 1 {
                if let Some(move_from) = {
                    // If there is a single proposal, pull it out, otherwise do nothing.
                    let tile = &mut self.map[row][col];
                    if tile.props.len() == 1 {
                        let prop = tile.props.pop().unwrap();
                        assert!(tile.elf.is_none());
                        Some(prop)
                    } else {
                        tile.props.clear();
                        None
                    }
                } {
                    // Move the lucky elf
                    moves += 1;
                    let elf = self.map[move_from.from_row][move_from.from_col].elf.take();
                    assert!(elf.is_some());
                    self.map[row][col].elf = elf;
                }
            }
        }
        moves
    }
    fn round(&mut self) -> usize {
        self.step1();
        self.step2()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    r: usize,
    c: usize,
}

fn read_map(f: &str) -> Map {
    Map::parse(
        &fs::read_to_string(f)
            .unwrap()
            .lines()
            .filter(|l| l.len() > 0)
            .map(|l| l.as_bytes())
            .into_iter()
            .collect::<Vec<&[u8]>>(),
    )
}
fn solve_part1(f: &str) -> usize {
    let mut m = read_map(f);
    let mut num_rounds = 0;
    while num_rounds < 10 {
        m.round();
        num_rounds += 1;
    }
    m.empty_tiles()
}

fn solve_part2(f: &str) -> usize {
    let mut m = read_map(f);
    let mut num_rounds = 1;
    while m.round() > 0 {
        num_rounds += 1;
    }
    num_rounds
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
