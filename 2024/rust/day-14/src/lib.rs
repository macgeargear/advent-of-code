pub mod part1;
pub mod part2;

use regex::Regex;

pub fn nums(s: &str) -> Vec<i64> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(s)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

#[derive(Debug)]
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

    fn new(px: i64, py: i64, vx: i64, vy: i64) -> Robot {
        Robot { px, py, vx, vy }
    }

    fn quadrant(&self) -> i64 {
        match (self.px > 50, self.py > 51) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        }
    }
}
