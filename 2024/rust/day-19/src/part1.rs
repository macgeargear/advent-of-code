use std::collections::*;

fn can_form(input: &str, vowels: &HashSet<&str>) -> bool {
    let n = input.len();
    let mut dp = vec![false; n + 1]; // dp(i) = can form input[0..i] using vowels
    dp[n] = true;

    for i in (0..n).rev() {
        for vowel in vowels {
            if i + vowel.len() <= n && &input[i..i + vowel.len()] == *vowel {
                dp[i] = dp[i + vowel.len()]
            }
            if dp[i] {
                break;
            }
        }
    }

    dp[0]
}

pub fn solve(input: &str) -> i32 {
    let input: Vec<&str> = input.split("\n\n").collect();
    let vowels: HashSet<&str> = input[0].split(", ").collect();
    let vocabs: Vec<&str> = input[1].trim().split("\n").collect();

    vocabs
        .iter()
        .filter(|vocab| can_form(vocab, &vowels))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(solve(input), 6);
    }
}
