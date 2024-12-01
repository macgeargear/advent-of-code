use std::collections::BinaryHeap;

pub fn solve(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut pq1: BinaryHeap<i32> = BinaryHeap::new();
    let mut pq2: BinaryHeap<i32> = BinaryHeap::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        pq1.push(nums[0]);
        pq2.push(nums[1]);
    }

    while !pq1.is_empty() && !pq2.is_empty() {
        let n1 = pq1.pop().unwrap();
        let n2 = pq2.pop().unwrap();
        sum += (n1 - n2).abs();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = solve(input);
        assert_eq!(result, 11);
    }
}
