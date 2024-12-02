fn valid_diff(num1: i32, num2: i32, asc: bool) -> bool {
    let diff: i32;
    if asc {
        diff = num2 - num1;
    } else {
        diff = num1 - num2;
    }
    if diff < 1 || diff > 3 {
        false
    } else {
        true
    }
}

fn check(num1: i32, num2: i32, asc: bool) -> bool {
    if asc {
        if num1 > num2 {
            false
        } else {
            valid_diff(num1, num2, asc)
        }
    } else {
        if num1 < num2 {
            false
        } else {
            valid_diff(num1, num2, asc)
        }
    }
}

pub fn solve(input: &str) -> i32 {
    let mut cnt = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();

        let mut valid = true;
        let asc = if nums[0] < nums[1] { true } else { false };
        for i in 0..nums.len() - 1 {
            if !check(nums[i], nums[i + 1], asc) {
                valid = false;
                break;
            }
        }

        if valid {
            cnt += 1;
            continue;
        }

        for i in 0..nums.len() {
            let mut valid = true;
            let filter_nums: Vec<i32> = nums
                .iter()
                .enumerate()
                .filter(|(j, _)| if i == *j { false } else { true })
                .map(|(_, &num)| num)
                .collect();
            let asc = if filter_nums[0] < filter_nums[1] {
                true
            } else {
                false
            };
            for j in 0..filter_nums.len() - 1 {
                if !check(filter_nums[j], filter_nums[j + 1], asc) {
                    valid = false;
                    break;
                }
            }

            if valid {
                cnt += 1;
                break;
            }
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
        assert_eq!(solve(input), 4);
    }
}
