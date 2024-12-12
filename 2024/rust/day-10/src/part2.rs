use std::collections::{HashSet, VecDeque};

fn valid(r: usize, c: usize, grid: &Vec<Vec<i32>>) -> bool {
    r >= 0 && r < grid.len() && c >= 0 && c < grid[0].len()
}

fn get_start_pos(grid: &Vec<Vec<i32>>) -> HashSet<(usize, usize)> {
    let mut start_pos = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                start_pos.insert((i, j));
            }
        }
    }

    start_pos
}

pub fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let starts = get_start_pos(&grid);

    let mut cnt = 0;

    for (r, c) in starts {
        let mut queue = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        queue.push_back((r, c));
        let mut t = 0;

        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();

            if grid[r][c] == 9 {
                // cnt += 1;
                t += 1;
            }

            for (dr, dc) in &dirs {
                let nr = (r as i32 + *dr) as usize;
                let nc = (c as i32 + *dc) as usize;

                if !valid(nr, nc, &grid) || grid[nr][nc] < grid[r][c]
                // || visited.contains(&(nr, nc))
                {
                    continue;
                }

                if grid[nr][nc] - grid[r][c] == 1 {
                    queue.push_back((nr, nc));
                    visited.insert((nr, nc));
                }
            }
        }
        println!("{}", t);
        cnt += t;
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "012345
123456
234567
345678
406789
567890";
        assert_eq!(solve(input), 227);
    }

    #[test]
    fn test_solve() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(solve(input), 81);
    }
}
