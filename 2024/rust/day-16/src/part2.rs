use std::collections::*;

struct Maze {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    dirs: Vec<(i32, i32)>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let (start, end) = Maze::get_start_end(&grid);

        Self {
            map: grid,
            start,
            end,
            dirs,
        }
    }

    fn get_start_end(map: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
        let mut start: (usize, usize) = (0, 0);
        let mut stop: (usize, usize) = (0, 0);

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                match map[i][j] {
                    'S' => start = (i, j),
                    'E' => stop = (i, j),
                    _ => (),
                }
            }
        }
        (start, stop)
    }

    fn valid(&self, r: i32, c: i32) -> bool {
        r < self.map.len() as i32
            && c < self.map[0].len() as i32
            && self.map[r as usize][c as usize] != '#'
    }

    fn find_all_min_path(&self) -> HashSet<(usize, usize)> {
        // (cost, row, col, direction, cur_path)
        let mut pq = BinaryHeap::new();
        let mut min_paths: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut min_cost = i32::MAX;
        let mut dist: HashMap<(usize, usize, i32), i32> = HashMap::new(); // (r, c, dir)

        pq.push((0, self.start.0, self.start.1, -1, vec![self.start]));
        dist.insert((self.start.0, self.start.1, -1), 0);

        while !pq.is_empty() {
            let (cost, r, c, d, cur_path) = pq.pop().unwrap();
            let cost = -cost; // max heap
            if (r, c) == self.end {
                if cost <= min_cost {
                    if cost < min_cost {
                        min_cost = cost;
                        min_paths.clear();
                    }
                    min_paths.push(cur_path);
                }
                continue;
            }

            let curr_best = dist.get(&(r, c, d)).unwrap_or(&i32::MAX);
            if cost > *curr_best {
                continue;
            }

            for (i, (dr, dc)) in self.dirs.iter().enumerate() {
                let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                if !self.valid(nr, nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);

                let new_cost = if i == d as usize {
                    cost + 1 // same direction
                } else {
                    cost + 1001 // change direction
                };

                if new_cost <= *dist.get(&(nr, nc, i as i32)).unwrap_or(&i32::MAX) {
                    dist.insert((nr, nc, i as i32), new_cost);
                    let mut new_path = cur_path.clone();
                    new_path.push((nr, nc));
                    pq.push((-new_cost, nr, nc, i as i32, new_path));
                }
            }
        }

        let mut result: HashSet<(usize, usize)> = HashSet::new();
        for path in min_paths {
            for pos in path {
                result.insert(pos);
            }
        }

        result
    }

    fn print(&self, result: &HashSet<(usize, usize)>) {
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if result.contains(&(i, j)) {
                    print!("O");
                } else {
                    print!("{}", self.map[i][j]);
                }
            }
            println!();
        }
    }
}

pub fn solve(input: &str) -> i32 {
    let maze = Maze::new(input);

    let result = maze.find_all_min_path();
    maze.print(&result);

    result.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(solve(input), 45);
    }
}
