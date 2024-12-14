use regex::Regex;
use std::{collections::HashSet, i64, ops::Rem};

#[derive(Debug, Clone)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn update(&mut self) {
        self.px = (self.px + self.vx).rem_euclid(101);
        self.py = (self.py + self.vy).rem_euclid(103);
    }
}

fn print_robot(robots: &Vec<Robot>) {
    let mut grid = vec![vec!['.'; 101]; 103];
    for robot in robots.iter() {
        grid[robot.py as usize][robot.px as usize] = '#';
    }
    for row in grid.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }
}

pub fn solve(input: &str) -> i64 {
    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Robot> = vec![];

    for line in input.lines() {
        if let Some(cap) = re.captures(line) {
            let px: i64 = cap[1].parse().unwrap();
            let py: i64 = cap[2].parse().unwrap();
            let vx: i64 = cap[3].parse().unwrap();
            let vy: i64 = cap[4].parse().unwrap();
            robots.push(Robot { px, py, vx, vy });
        }
    }

    let (X, Y) = (101, 103);

    for step in 1..=10_000 {
        let mut seen: HashSet<(i64, i64)> = HashSet::new();

        for robot in robots.iter_mut() {
            robot.update();
            seen.insert((robot.px, robot.py));
        }

        if seen.len() == robots.len() {
            println!("step: {}", step);
            print_robot(&robots);
            // if the result is seem like a christmas tree, the result is the step
        }
    }

    0
}
