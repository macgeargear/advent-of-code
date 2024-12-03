use std::{collections::VecDeque, i32};

pub fn solve(input: &str) -> i32 {
    let mut res = 0;
    let mut st: VecDeque<char> = VecDeque::new();
    let mut l = String::new();
    let mut r = String::new();
    let mut parsing_left = true;
    let chars: Vec<char> = input.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        let c = chars[i];

        // Check for "mul(" pattern
        if i + 3 < n && c == 'm' && &input[i..i + 4] == "mul(" {
            // Clear previous state if we find a new mul
            l.clear();
            r.clear();
            st.clear();
            parsing_left = true;
            st.push_back('(');
            i += 3; // Skip to the opening parenthesis
        } else if !st.is_empty() {
            match c {
                '(' => st.push_back('('),
                ',' => {
                    if st.back() == Some(&'(') {
                        parsing_left = false;
                    } else {
                        // Invalid state - reset
                        l.clear();
                        r.clear();
                        st.clear();
                        parsing_left = true;
                    }
                }
                ')' => {
                    if !l.is_empty() && !r.is_empty() {
                        if let (Ok(lnum), Ok(rnum)) = (l.parse::<i32>(), r.parse::<i32>()) {
                            res += lnum * rnum;
                        }
                        l.clear();
                        r.clear();
                        st.clear();
                        parsing_left = true;
                    }
                }
                d if d.is_digit(10) => {
                    if parsing_left {
                        l.push(d);
                    } else {
                        r.push(d);
                    }
                }
                ' ' => (), // Skip whitespace
                _ => {
                    // Reset state on invalid input
                    l.clear();
                    r.clear();
                    st.clear();
                    parsing_left = true;
                }
            }
        }
        i += 1;
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

    // Additional test cases
    #[test]
    fn test_with_spaces() {
        let input = "mul(2, 4)";
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn test_invalid_numbers() {
        let input = "mul(a,b)";
        assert_eq!(solve(input), 0);
    }
}
