use crate::{find_start_pos, valid};
use std::collections::HashSet;

fn can_trap(sr: i32, sc: i32, lines: &Vec<String>) -> i32 {
    let (mut r, mut c) = (sr, sc);
    let (row, col) = (lines.len(), lines[0].len());
    let mut pos: HashSet<(i32, i32, i32)> = HashSet::new();
    // up, right, down, left
    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut idir = 0;

    pos.insert((r, c, idir));

    loop {
        let (dr, dc) = dirs[idir as usize];
        let (nr, nc) = (r + dr, c + dc);

        println!("{} {} {} {}", nr, nc, row, col);
        if !valid(nr, nc, row, col) || lines[nr as usize].chars().nth(nc as usize).unwrap() == ' ' {
            break;
        }

        let next_char = lines[nr as usize].chars().nth(nc as usize).unwrap();
        if next_char == '#' {
            idir = (idir + 1) % 4;
            if pos.contains(&(r, c, idir as i32)) {
                return 1;
            }
            pos.insert((r, c, idir as i32));
            continue;
        }

        (r, c) = (nr, nc);
        if pos.contains(&(r, c, idir as i32)) {
            return 1;
        }

        pos.insert((r, c, idir as i32));
    }

    0
}

pub fn solve(input: &str) -> i32 {
    let (sr, sc) = find_start_pos(&input.lines().collect::<Vec<&str>>());
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut trap_cnt = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let c = lines[i].chars().nth(j).unwrap();
            if c == '#' || c == '^' {
                continue;
            }

            let mut tmp_lines = lines.clone();
            tmp_lines[i].replace_range(j..j + 1, "#");
            println!("try trap ({}, {})--------------------------------", i, j);
            trap_cnt += can_trap(sr as i32, sc as i32, &tmp_lines);
        }
    }

    trap_cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
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
        assert_eq!(solve(input), 6);
    }
}
