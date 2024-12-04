/*
M.S
.A.
M.S
---
M.M
.A.
S.S
---
S.M
.A.
S.M
---
S.S
.A.
M.M
*/

const PATTERNS: [[&str; 3]; 4] = [
    ["M.S", ".A.", "M.S"],
    ["M.M", ".A.", "S.S"],
    ["S.M", ".A.", "S.M"],
    ["S.S", ".A.", "M.M"],
];

pub fn solve(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let row = lines.len();
    let col = lines[0].len();
    let mut cnt: i32 = 0;

    for r in 0..row {
        for c in 0..col {
            cnt += is_valid(r, c, row, col, &lines) as i32;
        }
    }

    cnt
}

fn is_valid(r: usize, c: usize, row: usize, col: usize, lines: &[&str]) -> bool {
    for pattern in PATTERNS.iter() {
        let mut found = true;
        for i in 0..3 {
            for j in 0..3 {
                let cur_r = r + i;
                let cur_c = c + j;
                if cur_r >= row || cur_c >= col {
                    return false;
                }
                if pattern[i].as_bytes()[j] == b'.' {
                    continue;
                }
                if lines[cur_r].as_bytes()[cur_c] != pattern[i].as_bytes()[j] {
                    found = false;
                    break;
                }
                if !found {
                    break;
                }
            }
            println!();
        }
        if found {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(solve(input), 9);
    }
}
