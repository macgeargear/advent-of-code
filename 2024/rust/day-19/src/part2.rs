use std::collections::*;

fn cnt_form(input: &str, vowels: &HashSet<&str>) -> u64 {
    let n = input.len();
    let mut dp: Vec<u64> = vec![0; n + 1]; // dp(i) = can form input[0..i] using vowels
    dp[n] = 1;

    for i in (0..n).rev() {
        for vowel in vowels {
            if i + vowel.len() <= n && &input[i..i + vowel.len()] == *vowel {
                dp[i] += dp[i + vowel.len()]
            }
        }
    }

    dp[0]
}

pub fn solve(input: &str) -> u64 {
    let input: Vec<&str> = input.split("\n\n").collect();
    let vowels: HashSet<&str> = input[0].split(", ").collect();
    let vocabs: Vec<&str> = input[1].trim().split("\n").collect();

    vocabs
        .iter()
        .map(|vocab| cnt_form(vocab, &vowels))
        .sum::<u64>()
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
        assert_eq!(solve(input), 16);
    }
}
