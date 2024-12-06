pub mod part1;
pub mod part1_hashset;
pub mod part2;

pub fn valid(r: i32, c: i32, row: usize, col: usize) -> bool {
    r >= 0 && r < row as i32 && c >= 0 && c < col as i32
}

pub fn find_start_pos(lines: &Vec<&str>) -> (i32, i32) {
    for (i, line) in lines.iter().enumerate() {
        if let Some(j) = line.find('^') {
            return (i as i32, j as i32);
        }
    }
    (0, 0)
}
