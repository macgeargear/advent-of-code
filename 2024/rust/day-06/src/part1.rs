#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_boundary(r: usize, c: usize, row: usize, col: usize) -> bool {
    r == 0 || r == row - 1 || c == 0 || c == col - 1
}

fn find_obstacle(
    lines: &mut Vec<String>,
    r: usize,
    c: usize,
    row: usize,
    col: usize,
    dir: &Direction,
) -> (i32, i32) {
    match dir {
        Direction::Up => {
            for i in (0..r).rev() {
                if let Some(line) = lines.get_mut(i) {
                    let mut chars: Vec<char> = line.chars().collect();
                    if chars[c] == '#' {
                        *line = chars.into_iter().collect();
                        return (i as i32, c as i32);
                    }
                    chars[c] = 'X';
                    *line = chars.into_iter().collect();
                }
            }
            (-1, -1)
        }
        Direction::Down => {
            for i in r..row {
                if let Some(line) = lines.get_mut(i) {
                    let mut chars: Vec<char> = line.chars().collect();
                    if chars[c] == '#' {
                        *line = chars.into_iter().collect();
                        return (i as i32, c as i32);
                    }
                    chars[c] = 'X';
                    *line = chars.into_iter().collect();
                }
            }
            (-1, -1)
        }
        Direction::Left => {
            if let Some(line) = lines.get_mut(r) {
                let mut chars: Vec<char> = line.chars().collect();
                for i in (0..c).rev() {
                    if chars[i] == '#' {
                        *line = chars.into_iter().collect();
                        return (r as i32, i as i32);
                    }
                    chars[i] = 'X';
                }
                *line = chars.into_iter().collect();
            }
            (-1, -1)
        }
        Direction::Right => {
            if let Some(line) = lines.get_mut(r) {
                let mut chars: Vec<char> = line.chars().collect();
                for i in c..col {
                    if chars[i] == '#' {
                        *line = chars.into_iter().collect();
                        return (r as i32, i as i32);
                    }
                    chars[i] = 'X';
                }
                *line = chars.into_iter().collect();
            }
            (-1, -1)
        }
    }
}

fn find_start_pos(lines: &Vec<String>) -> (usize, usize) {
    let (mut r, mut c) = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        if let Some(j) = line.find('^') {
            r = i;
            c = j;
            break;
        }
    }
    (r, c)
}

pub fn solve(input: &str) -> i32 {
    let mut lines: Vec<String> = input.lines().map(String::from).collect();
    let (row, col) = (lines.len(), lines[0].len());
    let (mut r, mut c) = find_start_pos(&lines);
    lines[r] = lines[r].replace("^", "X");
    let mut dir = Direction::Up;
    let mut total: i32 = 0;

    while !is_boundary(r, c, row, col) {
        let (obs_r, obs_c) = find_obstacle(&mut lines, r, c, row, col, &dir);
        if obs_r != -1 && obs_c != -1 {
            println!("dir: {:?}, pos: ({}, {})", dir, r, c);
            match dir {
                Direction::Up => {
                    if obs_r < r as i32 {
                        dir = Direction::Right;
                        r = (obs_r + 1) as usize;
                    }
                }
                Direction::Down => {
                    if obs_r > r as i32 {
                        dir = Direction::Left;
                        r = (obs_r - 1) as usize;
                    }
                }
                Direction::Right => {
                    if obs_c > c as i32 {
                        dir = Direction::Down;
                        c = (obs_c - 1) as usize;
                    }
                }
                Direction::Left => {
                    if obs_c < c as i32 {
                        dir = Direction::Up;
                        c = (obs_c + 1) as usize;
                    }
                }
            }
        } else {
            match dir {
                Direction::Up => r = r.saturating_sub(1), // Move up safely
                Direction::Down => r = (r + 1).min(row - 1), // Move down safely
                Direction::Right => c = (c + 1).min(col - 1), // Move right safely
                Direction::Left => c = c.saturating_sub(1),
            }
        }
        for line in lines.iter() {
            println!("{}", line);
        }
    }

    for line in lines.iter() {
        println!("{}", line);
        total += line.chars().filter(|&c| c == 'X').count() as i32;
    }

    total
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
