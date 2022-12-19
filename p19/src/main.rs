use scanf::*;
use std::fs;
use std::ops::Index;
use std::ops::IndexMut;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_play_out() {
        let blueprint = Blueprint::parse("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.").unwrap();
        assert_eq!(
            play_out(&blueprint, &vec![]),
            Resources {
                resources: [0, 0, 0, 0]
            }
        );
        assert_eq!(
            play_out(&blueprint, &vec![None]),
            Resources {
                resources: [1, 0, 0, 0]
            }
        );
        assert_eq!(
            play_out(&blueprint, &vec![None, None]),
            Resources {
                resources: [2, 0, 0, 0]
            }
        );
        assert_eq!(
            play_out(&blueprint, &vec![None, None, Some(Clay)]),
            Resources {
                resources: [1, 0, 0, 0]
            }
        );
        assert_eq!(
            play_out(&blueprint, &vec![None, None, Some(Clay), None]),
            Resources {
                resources: [2, 1, 0, 0]
            }
        );
        assert_eq!(
            play_out(&blueprint, &vec![None, None, Some(Clay), None, Some(Clay)]),
            Resources {
                resources: [1, 2, 0, 0]
            }
        );
        assert_eq!(
            play_out(
                &blueprint,
                &vec![
                    None,
                    None,
                    Some(Clay),
                    None,
                    Some(Clay),
                    None,
                    Some(Clay),
                    None,
                    None,
                    None,
                    Some(Obs),
                    Some(Clay),
                    None,
                    None,
                    Some(Obs),
                    None,
                    None,
                    Some(Geo),
                    None,
                    None,
                    Some(Geo),
                    None,
                    None,
                    None,
                ]
            ),
            Resources {
                resources: [6, 41, 8, 9] // 17: resources: [3, 13, 8, 0]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("test.txt"), 33);
        //assert_eq!(solve_part1("test.txt"), 1725);
    }
    #[test]
    fn test_part2() {}
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Resources {
    resources: [usize; 4],
}
impl Resources {
    fn new() -> Self {
        Resources { resources: [0; 4] }
    }
    fn collect(&self, robots: &Robots) -> Self {
        let mut rv = self.resources.clone();
        for i in 0..4 {
            rv[i] += robots[i];
        }
        Resources { resources: rv }
    }
    fn collect_n(&self, robots: &Robots, n: usize) -> Self {
        let mut rv = self.resources.clone();
        for i in 0..4 {
            rv[i] += robots[i] * n;
        }
        Resources { resources: rv }
    }
    fn enough(&self, cost: &[usize; 4]) -> bool {
        for i in 0..4 {
            if cost[i] > self.resources[i] {
                return false;
            }
        }
        return true;
    }
    fn take(&self, cost: &[usize; 4]) -> Self {
        let mut rv = self.resources.clone();
        for i in 0..4 {
            rv[i] -= cost[i];
        }
        Resources { resources: rv }
    }
}

type Robots = [usize; 4];
struct Blueprint {
    index: usize,
    costs: [[usize; 4]; 4],
    limits: [usize; 4],
}

impl<T> Index<Type> for [T; 4] {
    type Output = T;
    fn index(&self, r: Type) -> &T {
        &self[r as usize]
    }
}

impl<T> IndexMut<Type> for [T; 4] {
    fn index_mut(&mut self, r: Type) -> &mut T {
        &mut self[r as usize]
    }
}

#[derive(Copy, Clone, Debug)]
enum Type {
    Ore = 0,
    Clay = 1,
    Obs = 2,
    Geo = 3,
}
use Type::*;

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
    fn can_build(&self, resources: &Resources) -> [bool; 4] {
        let mut rv = [false; 4];
        for r in [Ore, Clay, Obs, Geo] {
            if resources.enough(&self.costs[r]) {
                rv[r] = true;
            }
        }
        rv
    }
    fn build(&self, robot: Type, resources: &Resources, robots: &Robots) -> (Resources, Robots) {
        let mut rn = robots.clone();
        rn[robot] += 1;
        (resources.take(&self.costs[robot]), rn)
    }
}

fn step(
    step_limit: usize,
    resources: &Resources,
    robots: &Robots,
    steps: &mut Vec<Option<Type>>,
    blueprint: &Blueprint,
    limits: [usize; 4],
    f: &mut dyn FnMut(&Vec<Option<Type>>, &Resources),
) {
    /*
    if &play_out(blueprint, steps) != resources {
        print_steps(steps);
        //println!("robots: {:?}", robots);
        panic!(
            "have: {:?}, want: {:?}",
            resources,
            play_out(blueprint, steps)
        );
    }
     */
    let steps_remaining = step_limit - steps.len();
    if steps_remaining == 0 {
        f(&steps, resources);
        return;
    }
    let working_robots = robots.clone();
    let can_build = blueprint.can_build(resources);

    let mut evening_resources;
    let mut evening_robots;
    if can_build[Geo] {
        (evening_resources, evening_robots) = blueprint.build(Geo, &resources, robots);

        steps.push(Some(Geo));
        step(
            step_limit,
            &evening_resources.collect(&working_robots),
            &evening_robots,
            steps,
            blueprint,
            limits,
            f,
        );
        steps.pop();
    } else {
        let mut built_something = false;
        let tomorrows_resources = resources.collect(&working_robots);
        for robot in [Ore, Clay, Obs] {
            let max_i_d_ever_need = steps_remaining * limits[robot];
            let max_i_can_have = robots[robot] * steps_remaining + resources.resources[robot];
            let i_should_bother = max_i_can_have < max_i_d_ever_need;
            if can_build[robot] && robots[robot] < limits[robot] && i_should_bother {
                built_something = true;
                (evening_resources, evening_robots) =
                    blueprint.build(robot, &tomorrows_resources, robots);

                steps.push(Some(robot));
                step(
                    step_limit,
                    &evening_resources,
                    &evening_robots,
                    steps,
                    blueprint,
                    limits,
                    f,
                );
                steps.pop();
            }
        }
        let should_holdout =
            blueprint.can_build(&resources.collect_n(robots, steps_remaining - 1)) != can_build;
        if !built_something || should_holdout {
            steps.push(None);
            step(
                step_limit,
                &tomorrows_resources,
                &robots,
                steps,
                blueprint,
                limits,
                f,
            );
            steps.pop();
        }
    }
}

// Returs the resources available at the end of going through steps.
fn play_out(blueprint: &Blueprint, steps: &Vec<Option<Type>>) -> Resources {
    let mut resources = Resources::new();
    let mut robots = [1, 0, 0, 0];
    for step in steps {
        let working_robots = robots.clone();
        (resources, robots) = if let Some(robot) = step {
            blueprint.build(*robot, &resources, &robots)
        } else {
            (resources, robots)
        };
        resources = resources.collect(&working_robots);
    }
    resources
}

fn print_steps(steps: &Vec<Option<Type>>) {
    for s in steps {
        if s.is_some() {
            print!("{:?} ", s.unwrap());
        } else {
            print!("None ");
        }
    }
    println!();
}

fn solve_blueprint(blueprint: &Blueprint, step_limit: usize) -> usize {
    let resources = Resources::new();
    let robots = [1, 0, 0, 0];

    let mut max = 0;
    let mut count: usize = 0;
    let mut f = &mut |steps: &Vec<Option<Type>>, r: &Resources| {
        count += 1;
        //let g = play_out(blueprint, steps);
        if max < r.resources[Geo] {
            max = r.resources[Geo];
            println!("{}: count: {count}, new max: {max}", blueprint.index,);
            //print_steps(steps);
        }
    };
    step(
        step_limit,
        &resources,
        &robots,
        &mut vec![],
        blueprint,
        blueprint.limits,
        &mut f,
    );
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
    //println!("part 1: {}", solve_part1("input.txt"));
    //println!("part 2: {}", solve_part2("test.txt"));
    println!("part 2: {}", solve_part2("input.txt"));
}

