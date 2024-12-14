use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

fn extract_num(input: &str, pattern: &str) -> (i32, i32) {
    let re = Regex::new(pattern).unwrap();
    let cap = re.captures(input).unwrap();
    (cap[1].parse().unwrap(), cap[2].parse().unwrap())
}

fn tokens(
    prize: (i32, i32),
    cur: (i32, i32),
    btn_a: (i32, i32),
    btn_b: (i32, i32),
    used_a: i32,
    used_b: i32,
    memo: &mut HashMap<(i32, i32, i32, i32), i32>,
) -> i32 {
    if cur.0 > prize.0 || cur.1 > prize.1 || used_a > 100 || used_b > 100 {
        return i32::MAX;
    }

    if cur == prize {
        return used_a * 3 + used_b;
    }

    if let Some(&val) = memo.get(&(cur.0, cur.1, used_a, used_b)) {
        return val;
    }

    // btn_a
    let press_a = tokens(
        prize,
        (cur.0 + btn_a.0, cur.1 + btn_a.1),
        btn_a,
        btn_b,
        used_a + 1,
        used_b,
        memo,
    );

    // btn_b
    let press_b = tokens(
        prize,
        (cur.0 + btn_b.0, cur.1 + btn_b.1),
        btn_a,
        btn_b,
        used_a,
        used_b + 1,
        memo,
    );

    let res = min(press_a, press_b);
    memo.insert((cur.0, cur.1, used_a, used_b), res);
    res
}

pub fn solve(input: &str) -> i32 {
    let games: Vec<&str> = input.split("\n\n").collect();
    let mut totals: i32 = 0;

    for game in games.iter() {
        let mut memo: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();
        let game: Vec<&str> = game.trim().split("\n").collect();
        let btn_a = extract_num(game[0], r"X\+(\d+), Y\+(\d+)");
        let btn_b = extract_num(game[1], r"X\+(\d+), Y\+(\d+)");
        let prize = extract_num(game[2], r"X\=(\d+), Y\=(\d+)");
        let res = tokens(prize, (0, 0), btn_a, btn_b, 0, 0, &mut memo);

        if res != i32::MAX {
            totals += res;
        }
    }
    totals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(solve(input), 480);
    }
}
