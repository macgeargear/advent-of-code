use std::collections::HashSet;

fn find_antinode(
    an1: (i32, i32),
    an2: (i32, i32),
    grid: &Vec<Vec<char>>,
    anti_nodes: &mut HashSet<(i32, i32)>,
) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            // Skip non-empty cells
            if grid[r][c] == '#' {
                continue;
            }

            // check slop of 2 points
            let is_line =
                (r as i32 - an1.0) * (an2.1 - an1.1) == (c as i32 - an1.1) * (an2.0 - an1.0);
            true;

            if is_line {
                anti_nodes.insert((r as i32, c as i32));
            }
        }
    }
}

pub fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antenna: Vec<(i32, i32)> = Vec::new();

    // Collect all antenna positions
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '.' {
                antenna.push((r as i32, c as i32));
            }
        }
    }

    for i in 0..antenna.len() {
        for j in (i + 1)..antenna.len() {
            let an1 = antenna[i];
            let an2 = antenna[j];
            if grid[an1.0 as usize][an1.1 as usize] == grid[an2.0 as usize][an2.1 as usize] {
                find_antinode(an1, an2, &grid, &mut anti_nodes);
            }
        }
    }

    let mut grid2 = grid.clone();
    for antinode in anti_nodes.iter() {
        grid2[antinode.0 as usize][antinode.1 as usize] = '#';
    }

    for row in grid2.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    anti_nodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(solve(input), 34);
    }
}
