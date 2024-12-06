use std::collections::HashSet;

fn valid(r: i32, c: i32, row: usize, col: usize) -> bool {
    r >= 0 && r < row as i32 && c >= 0 && c < col as i32
}

fn find_start_pos(lines: &Vec<&str>) -> (i32, i32) {
    for (i, line) in lines.iter().enumerate() {
        if let Some(j) = line.find('^') {
            return (i as i32, j as i32);
        }
    }
    (0, 0)
}

pub fn solve(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut pos: HashSet<(i32, i32)> = HashSet::new();

    let (row, col) = (lines.len(), lines[0].len());
    let (mut r, mut c) = find_start_pos(&lines);

    // up, right, down, left
    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut idir = 0;

    pos.insert((r as i32, c as i32));

    loop {
        let (dr, dc) = dirs[idir];
        let (nr, nc) = (r as i32 + dr, c as i32 + dc);

        if !valid(nr, nc, row, col) {
            break;
        }

        let next_char = lines[nr as usize].chars().nth(nc as usize).unwrap();
        if next_char == '#' {
            idir = (idir + 1) % 4;
            continue;
        }

        (r, c) = (nr, nc);
        pos.insert((r, c));
    }

    pos.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve(input), 41);
    }
}