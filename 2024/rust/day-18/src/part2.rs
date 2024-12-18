use std::collections::*;

const GRID_SIZE: usize = 71;
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn valid(x: i32, y: i32) -> bool {
    x >= 0 && x < GRID_SIZE as i32 && y >= 0 && y < GRID_SIZE as i32
}

fn min_distance(grid: &Vec<Vec<char>>) -> i64 {
    let mut queue = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((0, 0, 0));
    seen.insert((0, 0));

    while !queue.is_empty() {
        let (cost, x, y) = queue.pop_front().unwrap();
        if (x, y) == (GRID_SIZE - 1, GRID_SIZE - 1) {
            return cost;
        }

        for (dx, dy) in DIRS.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if valid(nx, ny)
                && grid[ny as usize][nx as usize] != '#'
                && !seen.contains(&(nx as usize, ny as usize))
            {
                seen.insert((nx as usize, ny as usize));
                queue.push_back((cost + 1, nx as usize, ny as usize));
            }
        }
    }

    -1
}
pub fn solve(input: &str) -> (usize, usize) {
    let obs: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let mut grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE];

    for (i, (x, y)) in obs.iter().enumerate() {
        if i >= 1024 {
            break;
        }
        grid[*y][*x] = '#';
    }

    for row in grid.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }

    // min_distance(&grid)
    for i in 1024..obs.len() {
        let (x, y) = obs[i];
        grid[y][x] = '#';
        if min_distance(&grid) == -1 {
            return (x, y);
        }
        // grid[y][x] = '.';
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(solve(input), (6, 1));
    }
}
