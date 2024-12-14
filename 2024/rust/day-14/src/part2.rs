use std::collections::HashSet;

use crate::{nums, Robot};

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
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let line = nums(line);
        let robot = Robot::new(line[0], line[1], line[2], line[3]);
        robots.push(robot);
    }

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
