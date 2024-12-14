use std::collections::HashSet;

fn valid(r: i32, c: i32, m: i32, n: i32) -> bool {
    r >= 0 && r < m && c >= 0 && c < n
}

fn dfs_area(
    grid: &Vec<Vec<char>>,
    dirs: &Vec<(i32, i32)>,
    seen: &mut HashSet<(i32, i32, char)>,
    r: i32,
    c: i32,
    m: i32,
    n: i32,
    target: char,
) -> i32 {
    if seen.contains(&(r, c, target)) {
        return 0;
    }

    if !valid(r, c, m, n) || grid[r as usize][c as usize] != target {
        return 0;
    }

    seen.insert((r, c, target));
    let mut res = 0;

    if grid[r as usize][c as usize] == target {
        res += 1;
    }

    for (dr, dc) in dirs.iter() {
        let (nr, nc) = (r + dr, c + dc);
        res += dfs_area(grid, dirs, seen, nr, nc, m, n, target);
    }

    res
}

fn dfs_perimeter(
    grid: &Vec<Vec<char>>,
    dirs: &Vec<(i32, i32)>,
    seen: &mut HashSet<(i32, i32, char)>,
    r: i32,
    c: i32,
    m: i32,
    n: i32,
    target: char,
) -> i32 {
    if seen.contains(&(r, c, target)) {
        return 0;
    }

    if !valid(r, c, m, n) || grid[r as usize][c as usize] != target {
        return 1;
    }

    seen.insert((r, c, target));
    let mut res = 0;

    for (dr, dc) in dirs.iter() {
        let nr = r + dr;
        let nc = c + dc;
        res += dfs_perimeter(grid, dirs, seen, nr, nc, m, n, target);
    }

    res
}

pub fn solve(input: &str) -> i32 {
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut res = 0;
    let mut seen_a: HashSet<(i32, i32, char)> = HashSet::new();
    let mut seen_p: HashSet<(i32, i32, char)> = HashSet::new();
    for r in 0..m {
        for c in 0..n {
            let (r, c) = (r as usize, c as usize);
            let target = grid[r][c];
            let p = dfs_perimeter(&grid, &dirs, &mut seen_a, r as i32, c as i32, m, n, target);
            let a = dfs_area(&grid, &dirs, &mut seen_p, r as i32, c as i32, m, n, target);
            res += p * a;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = "AAAA
BBCD
BBCC
EEEC";

        assert_eq!(solve(input), 140);
    }

    #[test]
    fn test_solve2() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(solve(input), 772);
    }

    #[test]
    fn test_solve3() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(solve(input), 1930);
    }
}
