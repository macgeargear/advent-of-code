use std::collections::*;

fn valid(map: &Vec<Vec<char>>, r: i32, c: i32) -> bool {
    r < map.len() as i32 && c < map[0].len() as i32 && map[r as usize][c as usize] != '#'
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

pub fn solve(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    // (cost, row, col, direction)
    // direction:
    // 0 - right,
    // 1 - left,
    // 2 - down,
    // 3 - up,
    // -1 - start
    let mut pq: BinaryHeap<(i32, usize, usize, i32)> = BinaryHeap::new();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let (start, end) = get_start_end(&map);

    pq.push((0, start.0, start.1, -1));
    println!("{} {}", map.len(), map[0].len());
    println!("({:?}, {:?})", start, end);

    while !pq.is_empty() {
        let (cost, r, c, d) = pq.pop().unwrap();
        if (r, c) == end {
            return -cost;
        }
        for (dr, dc) in dirs.iter() {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if !valid(&map, nr, nc)
                || seen.contains(&(nr as usize, nc as usize))
                || map[nr as usize][nc as usize] == '#'
            {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            seen.insert((nr, nc));
            match (dr, dc) {
                // left
                (0, -1) => {
                    if d == 1 {
                        pq.push((cost - 1, nr, nc, 1));
                    } else {
                        pq.push((cost - 1001, nr, nc, 1));
                    }
                }
                // up
                (-1, 0) => {
                    if d == 3 {
                        pq.push((cost - 1, nr, nc, 3));
                    } else {
                        pq.push((cost - 1001, nr, nc, 3));
                    }
                }
                // right
                (0, 1) => {
                    if d == 0 {
                        pq.push((cost - 1, nr, nc, 0));
                    } else {
                        pq.push((cost - 1001, nr, nc, 0));
                    }
                }
                // down
                (1, 0) => {
                    if d == 2 {
                        pq.push((cost - 1, nr, nc, 2));
                    } else {
                        pq.push((cost - 1001, nr, nc, 2));
                    }
                }
                _ => {}
            }
        }
    }

    0
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
        assert_eq!(solve(input), 7036);
    }
}
