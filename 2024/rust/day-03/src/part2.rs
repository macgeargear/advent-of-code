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
    let mut valid = true;

    while i < n {
        let c = chars[i];

        if i + 3 < n && &input[i..i + 4] == "do()" {
            valid = true;
            i += 3;
        } else if i + 6 < n && &input[i..i + 7] == "don't()" {
            valid = false;
            i += 6;
        } else if i + 3 < n && &input[i..i + 4] == "mul(" {
            l.clear();
            r.clear();
            st.clear();
            parsing_left = true;
            st.push_back('(');
            i += 3;
        } else if !st.is_empty() {
            match c {
                '(' => st.push_back('('),
                ',' => {
                    if st.back() == Some(&'(') {
                        parsing_left = false;
                    } else {
                        l.clear();
                        r.clear();
                        st.clear();
                        parsing_left = true;
                    }
                }
                ')' => {
                    if !l.is_empty() && !r.is_empty() {
                        if let (Ok(lnum), Ok(rnum)) = (l.parse::<i32>(), r.parse::<i32>()) {
                            if valid {
                                res += lnum * rnum;
                            }
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
                ' ' => (),
                _ => {
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
        let input = "do()mul(2,4)";
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn test_dont_two_mul() {
        let input = "don't()mul(2,4)mul(2,4)";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn test_do_dont_do() {
        let input = "do()mul(2,4)don't()mul(3,3)do()mul(2,2)";
        assert_eq!(solve(input), 12); // 8 + 0 + 4
    }

    #[test]
    fn test_wrong_mul() {
        let input = "do()mul(2, 4]mul[2, 4)";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn test_solve() {
        let input = "do()mul(2,4)mul(3,7)mul(1,1)";
        assert_eq!(solve(input), 30); // 8 + 21 + 1
    }

    #[test]
    fn test_with_spaces() {
        let input = "do()mul(2, 4)";
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn test_invalid_numbers() {
        let input = "do()mul(a,b)";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn test_no_do() {
        let input = "mul(2,4)mul(3,3)";
        assert_eq!(solve(input), 0);
    }
}
