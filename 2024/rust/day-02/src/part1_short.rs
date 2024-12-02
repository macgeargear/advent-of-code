use crate::part1::check;

const MIN_DIFF: i32 = 1;
const MAX_DIFF: i32 = 3;

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect();
            let asc = nums[0] < nums[1];
            nums.windows(2).all(|nums| check(nums[0], nums[1], asc))
        })
        .count() as i32
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
