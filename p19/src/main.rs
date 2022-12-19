use scanf::*;
use std::fs;
use std::ops::Index;
use std::ops::IndexMut;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gradual_solve() {
        //let blueprint = Blueprint::parse("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.").unwrap();
        let blueprint = Blueprint::parse("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.").unwrap();

        let resources = Resources::new();
        let robots = [1, 0, 0, 0];

        let mut max = 0;
        let mut count: usize = 0;
        let f = &mut |steps: &Vec<Option<Type>>, r: &Resources| {
            count += 1;
            //let g = play_out(blueprint, steps);
            if max < r.resources[Geo] {
                max = r.resources[Geo];
                println!("{}: count: {count}, new max: {max}", blueprint.index,);
                print_steps(steps);
            }
        };

        gradual_solve(32, &resources, &robots, &mut vec![], &blueprint, f);
        println!("tried {} combinations, max: {max}", count);
        assert_eq!(max, 12);
    }

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

// How long will it take to make a robot given these robots and resources.
fn time_till_robot(
    goal_robot: Type,
    resources: &Resources,
    robots: &Robots,
    blueprint: &Blueprint,
) -> usize {
    let cost = blueprint.costs[goal_robot];
    let mut max = 0;
    for kind in [Ore, Clay, Obs, Geo] {
        let resources_missing = cost[kind].saturating_sub(resources.resources[kind]);
        let rate = robots[kind];
        if rate == 0 {
            continue;
        }
        let time = resources_missing / rate + if resources_missing % rate == 0 { 0 } else { 1 };
        max = max.max(time);
    }
    max
}

fn get_to_goal(
    goal_robot: Type,
    resources: &Resources,
    robots: &Robots,
    blueprint: &Blueprint,
) -> Vec<(Option<Type>, usize, Resources, Robots)> {
    let can_build = blueprint.can_build(resources);
    if can_build[goal_robot] {
        let (new_resources, new_robots) = blueprint.build(goal_robot, resources, robots);
        vec![(Some(goal_robot), 0, new_resources, new_robots)]
    } else {
        // How long would it take to get to goal if we try the various options
        let options = vec![None, Some(Ore), Some(Clay), Some(Obs)];
        let all_options = options
            .into_iter()
            .filter(|option| option.is_none() || can_build[option.unwrap()])
            .map(|option| {
                // We could build a robot of type Type. How long will it take to get to build a goal robot if we do that?
                if let Some(robot) = option {
                    let (new_resources, new_robots) = blueprint.build(robot, resources, robots);
                    (
                        option,
                        time_till_robot(goal_robot, &new_resources, &new_robots, blueprint),
                        new_resources,
                        new_robots,
                    )
                } else {
                    (
                        option,
                        time_till_robot(goal_robot, resources, robots, blueprint),
                        (*resources).clone(),
                        *robots,
                    )
                }
            })
            .map(|x| {
                println!( "building {:?} now will get us to {:?} in {} steps", x.0, goal_robot, x.1);
                x
            })
            .collect::<Vec<(Option<Type>, usize, Resources, Robots)>>();
        let min = all_options
            .iter()
            .min_by_key(|(_, time, _, _)| *time)
            .unwrap()
            .1;
        all_options.into_iter().filter(|x| x.1 == min).collect()
    }
}

// Tried solving by setting intermediate goals.
fn gradual_solve(
    step_limit: usize,
    resources: &Resources,
    robots: &Robots,
    steps: &mut Vec<Option<Type>>,
    blueprint: &Blueprint,
    f: &mut dyn FnMut(&Vec<Option<Type>>, &Resources),
) {
    let steps_remaining = step_limit - steps.len();
    if steps_remaining == 0 {
        //print_steps(steps);
        f(&steps, resources);
        return;
    }

    let options = if robots[Clay] == 0 {
        get_to_goal(Clay, resources, robots, blueprint)
    } else if robots[Obs] == 0 {
        get_to_goal(Obs, resources, robots, blueprint)
    } else {
        get_to_goal(Geo, resources, robots, blueprint)
    };

    for option in options.iter().take(1) {
        println!("building {:?}", option.0);
        steps.push(option.0);
        let new_robots = option.3;
        let new_resources = option.2.collect(robots);
        gradual_solve(step_limit, &new_resources, &new_robots, steps, blueprint, f);
        steps.pop();
    }
}

fn gradual_solve_blueprint(blueprint: &Blueprint, step_limit: usize) -> usize {
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
            print_steps(steps);
        }
    };

    println!("searching through: {:?}", blueprint.limits);
    gradual_solve(
        step_limit,
        &resources,
        &robots,
        &mut vec![],
        blueprint,
        &mut f,
    );
    println!("count: {} max: {}", count, max);
    max
}

fn main() {
    println!("part 1: {}", solve_part1("input.txt"));
    //println!("part 2: {}", solve_part2("test.txt"));
    //println!("part 2: {}", solve_part2("input.txt"));
}

fn step(
    step_limit: usize,
    resources: &Resources,
    robots: &Robots,
    steps: &mut Vec<Option<Type>>,
    blueprint: &Blueprint,
    limits: [usize; 4],
    hold_out_limit: usize,
    hold_out: usize,
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
            hold_out_limit,
            hold_out_limit,
            f,
        );
        steps.pop();
    } else {
        let mut built_something = false;
        let tomorrows_resources = resources.collect(&working_robots);
        for robot in [Ore, Clay, Obs] {
            if can_build[robot] && robots[robot] < limits[robot] {
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
                    hold_out_limit,
                    hold_out_limit,
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
                hold_out_limit,
                hold_out,
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
            print_steps(steps);
        }
    };

    println!("searching through: {:?}", blueprint.limits);
    for a in 1..=blueprint.limits[0] {
        for b in 1..=blueprint.limits[1] {
            if a + b > step_limit - 3 {
                continue;
            }
            for c in 1..=blueprint.limits[2].min(21 - a - b) {
                step(
                    step_limit,
                    &resources,
                    &robots,
                    &mut vec![],
                    blueprint,
                    [a, b, c, 1000],
                    1,
                    1,
                    &mut f,
                );
            }
        }
    }
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
        .map(|blueprint| blueprint.index * gradual_solve_blueprint(&blueprint, 24))
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
