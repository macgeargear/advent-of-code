use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut res: i32 = 0;

    for cap in re.captures_iter(input) {
        let l: i32 = cap[1].parse().unwrap_or(0);
        let r: i32 = cap[2].parse().unwrap_or(0);
        res += l * r;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_mul() {
        let input = "mul(2,4)";
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn test_two_mul() {
        let input = "mul(2,4)mul(2,4)";
        assert_eq!(solve(input), 16);
    }

    #[test]
    fn test_wrong_mul() {
        let input = "mul(2, 4]mul[2, 4)";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn test_solve() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve(input), 161);
    }
}
