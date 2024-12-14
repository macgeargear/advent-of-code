use crate::{nums, Robot};

pub fn solve(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut robots: Vec<Robot> = vec![];
    for line in lines {
        let line = nums(line);
        let robot = Robot::new(line[0], line[1], line[2], line[3]);
        robots.push(robot);
    }
    let mut q = (0, 0, 0, 0);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.update();
        }
    }

    for robot in robots {
        if robot.px == 50 || robot.py == 51 {
            continue;
        }
        match robot.quadrant() {
            0 => q.0 += 1,
            1 => q.1 += 1,
            2 => q.2 += 1,
            3 => q.3 += 1,
            _ => (),
        }
    }

    q.0 * q.1 * q.2 * q.3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(solve(input), 12);
    }
}
