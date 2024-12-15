struct Robot {
    r: i32,
    c: i32,
}

impl Robot {
    fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }

    fn push_boxes(
        &self,
        map: &mut Vec<Vec<char>>,
        r: i32,
        c: i32,
        dr: i32,
        dc: i32,
        m: i32,
        n: i32,
    ) -> bool {
        let (nr, nc) = (r + dr, c + dc);
        if !valid(&map, r, c, m, n) {
            return false;
        }

        if map[nr as usize][nc as usize] == '.' {
            map[nr as usize][nc as usize] = 'O';
            map[r as usize][c as usize] = '.';
            return true;
        }

        if map[nr as usize][nc as usize] == 'O' {
            if self.push_boxes(map, nr, nc, dr, dc, m, n) {
                map[nr as usize][nc as usize] = 'O';
                map[r as usize][c as usize] = '.';
                return true;
            }
        }

        false
    }

    fn update(&mut self, dir: char, map: &mut Vec<Vec<char>>, m: i32, n: i32) {
        let (dr, dc) = match dir {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => (0, 0),
        };

        let (nr, nc) = (self.r + dr, self.c + dc);
        if valid(&map, nr, nc, m, n) {
            if map[nr as usize][nc as usize] == 'O' {
                if self.push_boxes(map, nr, nc, dr, dc, m, n) {
                    map[self.r as usize][self.c as usize] = '.';
                    self.r = nr;
                    self.c = nc;
                    map[self.r as usize][self.c as usize] = '@';
                }
            } else {
                map[self.r as usize][self.c as usize] = '.';
                self.r = nr;
                self.c = nc;
                map[self.r as usize][self.c as usize] = '@';
            }
        }
    }
}

fn valid(grid: &Vec<Vec<char>>, r: i32, c: i32, m: i32, n: i32) -> bool {
    0 <= r && r < m && 0 <= c && c < n && grid[r as usize][c as usize] != '#'
}

fn get_start_pos(map: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                return (i as i32, j as i32);
            }
        }
    }
    (0, 0)
}

pub fn solve(input: &str) -> i32 {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut map: Vec<Vec<char>> = input[0]
        .split("\n")
        .map(|x| x.trim().chars().collect())
        .collect();
    let (m, n) = (map.len(), map[0].len());
    let dirs: Vec<char> = input[1].split("\n").collect::<String>().chars().collect();
    let (sr, sc) = get_start_pos(&map);
    let mut robot = Robot::new(sr, sc);
    let mut total = 0;

    println!("before");
    for i in 0..m {
        for j in 0..n {
            print!("{}", map[i][j]);
        }
        println!();
    }

    for dir in dirs {
        robot.update(dir, &mut map, m as i32, n as i32);
    }

    println!("after");
    for i in 0..m {
        for j in 0..n {
            print!("{}", map[i][j]);
        }
        println!();
    }

    for i in 0..m {
        for j in 0..n {
            if map[i][j] == 'O' {
                total += 100 * i as i32 + j as i32;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(solve(input), 10092);
    }

    #[test]
    fn small_test() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

        assert_eq!(solve(input), 2028);
    }
}
