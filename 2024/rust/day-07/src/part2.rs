fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap().parse().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|v| v.parse().unwrap())
                .collect();
            (key, values)
        })
        .collect()
}

pub fn solve(input: &str) -> i64 {
    let lines = parse_input(input);
    let mut total: i64 = 0;

    for (key, values) in lines.iter() {
        println!("{}: {:?}", key, values);
        if dfs(1, values[0], *key, values) {
            total += key;
        }
    }

    total
}

fn dfs(i: usize, cur: i64, target: i64, nums: &Vec<i64>) -> bool {
    if i == nums.len() {
        if cur == target {
            return true;
        } else {
            return false;
        }
    }

    let plus = dfs(i + 1, cur + nums[i], target, nums);
    let mul = dfs(i + 1, cur * nums[i], target, nums);
    let concat = dfs(
        i + 1,
        concat_int(&cur.to_string(), &nums[i].to_string()),
        target,
        nums,
    );

    plus || mul || concat
}

fn concat_int(a: &str, b: &str) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve(input), 11387);
    }
}
