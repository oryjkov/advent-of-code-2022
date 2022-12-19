use scanf::*;
use std::fs;
use std::ops::Index;
use std::ops::IndexMut;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 33);
        assert_eq!(solve_part1("input.txt"), 1725);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("test.txt"), 56 * 62);
        assert_eq!(solve_part2("input.txt"), 15510);
    }
}

type Robots = [usize; 4];
type Resources = [usize; 4];
type Cost = [usize; 4];

impl<T> Index<Material> for [T; 4] {
    type Output = T;
    fn index(&self, r: Material) -> &T {
        &self[r as usize]
    }
}

impl<T> IndexMut<Material> for [T; 4] {
    fn index_mut(&mut self, r: Material) -> &mut T {
        &mut self[r as usize]
    }
}

#[derive(Clone)]
struct State {
    resources: Resources,
    robots: Robots,
}
impl State {
    fn collect(resources: &Resources, robots: &Robots) -> Resources {
        let mut rv = resources.clone();
        for i in 0..4 {
            rv[i] += robots[i];
        }
        rv
    }
    fn take(resources: &Resources, cost: &[usize; 4]) -> Resources {
        let mut rv = resources.clone();
        for i in 0..4 {
            rv[i] -= cost[i];
        }
        rv
    }
    fn add_robot(robots: &Robots, robot: Material) -> Robots {
        let mut rv = robots.clone();
        rv[robot] += 1;
        rv
    }

    fn build(&self, maybe_robot: Option<Material>, blueprint: &Blueprint) -> Option<State> {
        match maybe_robot {
            None => Some(State {
                resources: Self::collect(&self.resources, &self.robots),
                robots: self.robots,
            }),
            Some(robot) => {
                if !blueprint.can_build(robot, &self.resources) {
                    return None;
                }
                Some(State {
                    resources: Self::collect(
                        &Self::take(&self.resources, &blueprint.costs[robot]),
                        &self.robots,
                    ),
                    robots: Self::add_robot(&self.robots, robot),
                })
            }
        }
    }
}

// State in the search tree.
struct Step {
    state: State,
    action: Option<Material>,
}

fn step(
    step_limit: usize,
    state: &State,
    steps: &mut Vec<Step>,
    blueprint: &Blueprint,
    report_fn: &mut dyn FnMut(&Vec<Step>),
) {
    let steps_remaining = step_limit - steps.len();
    if steps_remaining == 0 {
        // Push the state at the end of _this_ step
        steps.push(Step {
            state: state.clone(),
            action: None,
        });
        report_fn(&steps);
        steps.pop();
        return;
    }
    if let Some(next_state) = state.build(Some(Geo), &blueprint) {
        steps.push(Step {
            state: state.clone(),
            action: Some(Geo),
        });
        step(step_limit, &next_state, steps, blueprint, report_fn);
        steps.pop();
    } else {
        for robot in [Ore, Clay, Obs] {
            let max_i_d_ever_need = steps_remaining * blueprint.limits[robot];
            let max_i_can_have = state.robots[robot] * steps_remaining + state.resources[robot];
            let i_should_bother = max_i_can_have < max_i_d_ever_need;
            if !i_should_bother {
                continue;
            }
            if state.robots[robot] < blueprint.limits[robot] {
                if let Some(next_state) = state.build(Some(robot), &blueprint) {
                    // Skip building a robot that we could have built during the last step.
                    if steps.len() > 0
                        && steps[steps.len() - 1].action.is_none()
                        && blueprint.can_build(robot, &steps[steps.len() - 1].state.resources)
                    {
                        continue;
                    }

                    steps.push(Step {
                        state: state.clone(),
                        action: Some(robot),
                    });
                    step(step_limit, &next_state, steps, blueprint, report_fn);
                    steps.pop();
                }
            }
        }

        let next_state = &state.build(None, &blueprint).unwrap();
        steps.push(Step {
            state: state.clone(),
            action: None,
        });
        step(step_limit, &next_state, steps, blueprint, report_fn);
        steps.pop();
    }
}

struct Blueprint {
    index: usize,
    costs: [Cost; 4],
    // Highest value for any robot (max consumption of its resource).
    limits: [usize; 4],
}

#[derive(Copy, Clone, Debug)]
enum Material {
    Ore = 0,
    Clay = 1,
    Obs = 2,
    Geo = 3,
}
use Material::*;

impl Blueprint {
    fn parse(s: &str) -> Option<Self> {
        let mut index: usize = 0;
        let mut ore_per_ore: usize = 0;
        let mut ore_per_clay: usize = 0;
        let mut ore_per_obs: usize = 0;
        let mut clay_per_obs: usize = 0;
        let mut ore_per_geo: usize = 0;
        let mut obs_per_geo: usize = 0;
        sscanf!(s, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            index, ore_per_ore, ore_per_clay, ore_per_obs, clay_per_obs, ore_per_geo, obs_per_geo,).ok()?;
        let costs = [
            [ore_per_ore, 0, 0, 0],
            [ore_per_clay, 0, 0, 0],
            [ore_per_obs, clay_per_obs, 0, 0],
            [ore_per_geo, 0, obs_per_geo, 0],
        ];
        println!("{:?}", costs);
        let limits = [
            ore_per_ore
                .max(ore_per_clay)
                .max(ore_per_obs)
                .max(ore_per_geo),
            clay_per_obs,
            obs_per_geo,
            1000,
        ];
        Some(Blueprint {
            index,
            costs,
            limits,
        })
    }
    fn can_build(&self, robot: Material, resources: &Resources) -> bool {
        for i in 0..4 {
            if resources[i] < self.costs[robot][i] {
                return false;
            }
        }
        true
    }
}

fn print_steps(steps: &Vec<Step>) {
    for s in steps {
        if s.action.is_some() {
            print!("{:?} ", s.action.unwrap());
        } else {
            print!("None ");
        }
    }
    println!();
}

fn solve_blueprint(blueprint: &Blueprint, step_limit: usize) -> usize {
    let state = State {
        resources: [0; 4],
        robots: [1, 0, 0, 0],
    };

    let mut max = 0;
    let mut count: usize = 0;
    let mut report_fn = &mut |s: &Vec<Step>| {
        count += 1;
        let r = s[s.len() - 1].state.resources[Geo];
        if max < r {
            max = r;
            println!("{}: count: {count}, new max: {max}", blueprint.index,);
            print_steps(s);
        }
    };
    step(step_limit, &state, &mut vec![], blueprint, &mut report_fn);
    println!("count: {} max: {}", count, max);
    max
}

fn solve_part1(f: &str) -> usize {
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        //.skip(1)
        .map(|l| Blueprint::parse(l).unwrap())
        .map(|blueprint| blueprint.index * solve_blueprint(&blueprint, 24))
        .sum()
}

fn solve_part2(f: &str) -> usize {
    fs::read_to_string(f)
        .unwrap()
        .lines()
        .filter(|l| l.len() > 0)
        .take(3)
        .map(|l| Blueprint::parse(l).unwrap())
        .map(|blueprint| solve_blueprint(&blueprint, 32))
        .product()
}

fn main() {
    //println!("part 1: {}", solve_part1("test.txt"));
    println!("part 1: {}", solve_part1("input.txt"));
    //println!("part 2: {}", solve_part2("test.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}
