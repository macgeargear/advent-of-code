use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

fn extract_num(input: &str, pattern: &str) -> (i64, i64) {
    let re = Regex::new(pattern).unwrap();
    let cap = re.captures(input).unwrap();
    (cap[1].parse().unwrap(), cap[2].parse().unwrap())
}

fn tokens(
    prize: (i64, i64),
    cur: (i64, i64),
    btn_a: (i64, i64),
    btn_b: (i64, i64),
    used_a: i64,
    used_b: i64,
    memo: &mut HashMap<(i64, i64, i64, i64), i64>,
) -> i64 {
    if cur.0 > prize.0 || cur.1 > prize.1 {
        return i64::MAX;
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

pub fn solve(input: &str) -> i64 {
    let games: Vec<&str> = input.split("\n\n").collect();
    let mut totals: i64 = 0;

    for game in games.iter() {
        let mut memo: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        let game: Vec<&str> = game.trim().split("\n").collect();
        let btn_a = extract_num(game[0], r"X\+(\d+), Y\+(\d+)");
        let btn_b = extract_num(game[1], r"X\+(\d+), Y\+(\d+)");
        let prize = extract_num(game[2], r"X\=(\d+), Y\=(\d+)");
        let err = 10000000000000;
        let res = tokens(
            (prize.0 + err, prize.1 + err),
            (0, 0),
            btn_a,
            btn_b,
            0,
            0,
            &mut memo,
        );

        if res != i64::MAX {
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
