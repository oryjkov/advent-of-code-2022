use std::fs;
#[derive(Debug)]
enum Instr {
    Nop,
    Add(i32),
}
impl Instr {
    fn len(&self) -> usize {
        match self {
            Instr::Nop => 1,
            Instr::Add(_) => 2,
        }
    }
    fn apply(&self, x: &mut i32) {
        match self {
            Instr::Add(i) => *x += i,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(13140, solve_p1("test.txt"));
    }
}

struct State {
    instr: Instr,
    sub_i: usize,
    x: i32,
}
struct StateMachine<I> {
    iter: I,
    state: State,
}
impl<I> Iterator for StateMachine<I>
where
    I: Iterator<Item = Instr>,
{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state.sub_i < self.state.instr.len() {
            self.state.sub_i += 1;
            let old_x = self.state.x;
            self.state.instr.apply(&mut self.state.x);
            Some(old_x)
        } else if let Some(instr) = self.iter.next() {
            self.state.instr = instr;
            self.state.sub_i = 1;
            Some(self.state.x)
        } else {
            None
        }
    }
}
trait StateMachineIterator: Iterator + Sized {
    fn process_instrs(self) -> StateMachine<Self> {
        StateMachine {
            iter: self,
            state: State {
                instr: Instr::Nop,
                sub_i: 1,
                x: 1,
            },
        }
    }
}
impl<I: Iterator<Item = Instr>> StateMachineIterator for I {}

fn parse_input(f: &str) -> Vec<Instr> {
    fs::read_to_string(f)
        .expect("read fail")
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|s| {
            let i: Vec<&str> = s.split_whitespace().collect();
            if i.len() == 1 {
                Instr::Nop
            } else {
                Instr::Add(i[1].parse().unwrap())
            }
        })
        .collect()
}

fn solve_p1(f: &str) -> i32 {
    parse_input(f)
        .into_iter()
        .process_instrs()
        .enumerate()
        .filter(|(cycle, _)| cycle % 40 == 20)
        .map(|(cycle, x)| x * cycle as i32)
        .sum()
}

fn solve_p2(f: &str) {
    parse_input(f)
        .into_iter()
        .process_instrs()
        .enumerate()
        .map(|(cycle, x)| {
            let col = cycle % 40;
            let d = if (col as i32 - x).abs() <= 1 {
                "#"
            } else {
                "."
            };
            print!("{d}");
            if col == 39 {
                println!()
            }
        })
        .count();
}

fn main() {
    let f = "input.txt";
    println!("part1: {}", solve_p1(f));
    println!("part2:");
    solve_p2(f);
}
