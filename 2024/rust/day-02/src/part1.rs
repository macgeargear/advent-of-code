const MIN_DIFF: i32 = 1;
const MAX_DIFF: i32 = 3;

pub fn check(num1: i32, num2: i32, asc: bool) -> bool {
    let diff = if asc { num2 - num1 } else { num1 - num2 };
    diff >= MIN_DIFF && diff <= MAX_DIFF
}

pub fn solve(input: &str) -> i32 {
    let mut cnt = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();
        let asc = nums[0] < nums[1];
        let mut valid = true;
        for i in 0..nums.len() - 1 {
            if !check(nums[i], nums[i + 1], asc) {
                valid = false;
                break;
            }
        }

        if valid {
            cnt += 1;
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(solve(input), 2);
    }
}
