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
pub fn solve(input: &str) -> i64 {
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

    min_distance(&grid)
}
