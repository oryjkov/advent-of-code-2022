use std::fs;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test0.txt"), 10);
        assert_eq!(solve_part1("test.txt"), 64);
        assert_eq!(solve_part1("input.txt"), 4390);
    }
    #[test]
    fn test_part2() {}
}

fn solve_part1(f: &str) -> i32 {
    let mut coords: Vec<[i32; 3]> = fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| {
            let vs = l
                .split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            [vs[0], vs[1], vs[2]]
        })
        .collect();
    // adjust input to be > 0 in each coordinate
    let max = loop {
        let max = coords
            .iter()
            .flatten()
            .fold(-100, |accum, item| accum.max(*item));
        let min = coords
            .iter()
            .flatten()
            .fold(100, |accum, item| accum.min(*item));
        if min > 0 {
            break max;
        }
        coords.iter_mut().for_each(|c| {
            (*c)[0] += 1 - min;
            (*c)[1] += 1 - min;
            (*c)[2] += 1 - min;
        });
    };
    let len = max as usize + 2;
    let mut g = vec![vec![vec![0u8; len]; len]; len];
    coords
        .iter()
        .for_each(|c| g[c[0] as usize][c[1] as usize][c[2] as usize] = 1);
    let mut neibs = 0i32;
    let ds = vec![
        [0, 0, 1],
        [0, 0, -1],
        [0, 1, 0],
        [0, -1, 0],
        [1, 0, 0],
        [-1, 0, 0],
    ];
    for x in 0..len {
        for y in 0..len {
            for z in 0..len {
                if g[x][y][z] == 1 {
                    ds.iter().for_each(|d| {
                        neibs += g[(x as isize + d[0]) as usize][(y as isize + d[1]) as usize]
                            [(z as isize + d[2]) as usize] as i32
                    });
                }
            }
        }
    }
    println!("num cubes: {}, shared sides*2: {}", coords.len(), neibs);
    (coords.len() * 6) as i32 - neibs
}

/*
fn solve_part2(f: &str) -> i32 {
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .count();
    -1
}
 */

fn main() {
    println!("part 1: {}", solve_part1("test.txt"));
    //println!("part 2: {}", solve_part2("input.txt"));
}
