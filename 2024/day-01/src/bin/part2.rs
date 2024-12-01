use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2");
    let result = part2(input);

    println!("{}", result);
}

fn part2(input: &str) -> i32 {
    let mut right_map: HashMap<i32, i32> = HashMap::new();
    let mut left: Vec<i32> = vec![];
    let mut res: i32 = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        left.push(nums[0]);
        if let Some(v) = right_map.get_mut(&nums[1]) {
            *v += 1;
        } else {
            right_map.insert(nums[1], 1);
        }
    }

    for e in left.iter() {
        if let Some(v) = right_map.get_mut(e) {
            println!("{} * {} = {}", e, v, *e * *v);
            res += *e * *v;
        }
    }

    res
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
        let result = part2(input);
        assert_eq!(result, 31);
    }
}
