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
    let mut pq: BinaryHeap<(i32, usize, usize, i32, Vec<(usize, usize)>)> = BinaryHeap::new();
    let mut min_paths: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut min_cost = i32::MAX;
    let mut dist: HashMap<(usize, usize, i32), i32> = HashMap::new();
    let (start, end) = get_start_end(&map);

    pq.push((0, start.0, start.1, -1, vec![start]));
    dist.insert((start.0, start.1, -1), 0);

    while let Some((cost, r, c, d, path)) = pq.pop() {
        let cost = -cost;

        if (r, c) == end {
            if cost <= min_cost {
                if cost < min_cost {
                    min_cost = cost;
                    min_paths.clear();
                }
                min_paths.push(path);
            }
            continue;
        }

        let curr_best = dist.get(&(r, c, d)).unwrap_or(&i32::MAX);
        if cost > *curr_best {
            continue;
        }

        for (i, (dr, dc)) in dirs.iter().enumerate() {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if !valid(&map, nr, nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let new_cost = if i == d as usize {
                cost + 1
            } else {
                cost + 1001
            };

            if new_cost <= *dist.get(&(nr, nc, i as i32)).unwrap_or(&i32::MAX) {
                dist.insert((nr, nc, i as i32), new_cost);
                let mut new_path = path.clone();
                new_path.push((nr, nc));
                pq.push((-new_cost, nr, nc, i as i32, new_path));
            }
        }
    }

    // Combine all minimum cost paths into result
    let mut result: HashSet<(usize, usize)> = HashSet::new();
    for path in min_paths {
        for pos in path {
            result.insert(pos);
        }
    }

    // Print the result map
    // for i in 0..map.len() {
    //     for j in 0..map[0].len() {
    //         if result.contains(&(i, j)) {
    //             print!("O");
    //         } else {
    //             print!("{}", map[i][j]);
    //         }
    //     }
    //     println!();
    // }

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
