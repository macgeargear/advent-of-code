use regex::Regex;

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

pub fn solve(input: &str) -> i32 {
    let re = Regex::new(r"(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut robots: Vec<Robot> = vec![];
    for line in lines {
        let cap = re.captures(line).unwrap();
        let px: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let py: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let vx: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
        let vy: i32 = cap.get(4).unwrap().as_str().parse().unwrap();
        robots.push(Robot { px, py, vx, vy })
    }
    let times = 100;
    let (X, Y) = (101, 103);
    let mut q = (0, 0, 0, 0);

    for _ in 0..times {
        for robot in robots.iter_mut() {
            robot.px = (robot.px + 101 + robot.vx) % X;
            robot.py = (robot.py + 103 + robot.vy) % Y;
        }
    }

    for robot in robots {
        if robot.px == 50 || robot.py == 51 {
            continue;
        }
        if robot.px > 50 && robot.py > 51 {
            q.0 += 1;
        }
        if robot.px > 50 && robot.py < 51 {
            q.1 += 1;
        }
        if robot.px < 50 && robot.py > 51 {
            q.2 += 1;
        }
        if robot.px < 50 && robot.py < 51 {
            q.3 += 1;
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
