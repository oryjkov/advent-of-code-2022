use gcollections::ops::*;
use interval::interval_set::*;
use std::fs;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(
            to_iset(
                &vec![SB {
                    sensor: (1, 0),
                    beacon: (2, 0)
                }],
                2,
                true
            )
            .size(),
            0
        );
        assert_eq!(
            to_iset(
                &vec![SB {
                    sensor: (1, 0),
                    beacon: (2, 0)
                }],
                1,
                true
            )
            .size(),
            1
        );
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt", 10), 26);
        assert_eq!(solve_part1("input.txt", 2_000_000), 4725496);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("test.txt", 20), 56000011);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SB {
    // Coordinates as x,y
    sensor: (i32, i32),
    beacon: (i32, i32),
}
fn to_iset(inp: &Vec<SB>, ml: i32, remove: bool) -> IntervalSet<i32> {
    let i = inp
        .iter()
        .map(|sb| {
            let budget = (sb.sensor.0 - sb.beacon.0).abs() + (sb.sensor.1 - sb.beacon.1).abs();
            let dy = (ml - sb.sensor.1).abs();
            let rv = (sb.sensor.0 - (budget - dy), sb.sensor.0 + (budget - dy));
            //println!("budget: {}, dy: {}, i: {:?}, sb: {:?}", budget, dy, rv, sb);
            rv
        })
        .filter(|i| i.1 >= i.0)
        .map(|i| vec![i].to_interval_set())
        .fold(IntervalSet::empty(), |set, interval| set.union(&interval));

    if remove {
        let out = inp
            .iter()
            .filter(|sb| sb.beacon.1 == ml)
            .map(|sb| IntervalSet::singleton(sb.beacon.0))
            .fold(IntervalSet::empty(), |set, interval| set.union(&interval));
        let out = inp
            .iter()
            .filter(|sb| sb.sensor.1 == ml)
            .map(|sb| IntervalSet::singleton(sb.beacon.0))
            .fold(out, |set, interval| set.union(&interval));

        i.difference(&out)
    } else {
        i
    }
}

fn read_input(f: &str) -> Vec<SB> {
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| {
            let mut sx = 0i32;
            let mut sy = 0i32;
            let mut bx = 0i32;
            let mut by = 0i32;
            scanf::sscanf!(
                l,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                sx,
                sy,
                bx,
                by
            )
            .unwrap();
            SB {
                sensor: (sx, sy),
                beacon: (bx, by),
            }
        })
        .collect()
}

fn solve_part1(f: &str, ml: i32) -> i32 {
    to_iset(&read_input(f), ml, true).size() as i32
}

fn solve_part2(f: &str, mb: i32) -> i64 {
    let inp = read_input(f);
    for row in 0..mb {
        let i = to_iset(&inp, row, false);
        let int = vec![(0, mb)].to_interval_set().difference(&i);
        if int.is_singleton() {
            return int.lower() as i64 * 4000000 + row as i64;
        }
    }
    -1
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt", 2_000_000));
    println!("part 2: {}", solve_part2("input.txt", 4_000_000));
}
