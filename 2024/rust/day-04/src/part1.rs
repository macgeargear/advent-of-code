#[derive(Debug, Clone, Copy)]
enum Direction {
    // Up = (-1, 0),
    // Down = (1, 0),
    // Left = (0, -1),
    // Right = (0, 1),
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

const PATTERNS: &[u8] = b"XMAS";

pub fn solve(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let row = lines.len();
    let col = lines[0].len();
    let mut cnt: i32 = 0;

    for r in 0..row {
        for c in 0..col {
            cnt += search(r, c, row, col, &lines);
        }
    }

    cnt
}

fn search(r: usize, c: usize, row: usize, col: usize, lines: &[&str]) -> i32 {
    let directions = [
        Direction::Top,
        Direction::Bottom,
        Direction::Left,
        Direction::Right,
        Direction::TopLeft,
        Direction::TopRight,
        Direction::BottomLeft,
        Direction::BottomRight,
    ];

    directions
        .iter()
        .filter(|&&dir| search_in_direction(r, c, row, col, lines, dir))
        .count() as i32
}

fn search_in_direction(
    r: usize,
    c: usize,
    row: usize,
    col: usize,
    lines: &[&str],
    dir: Direction,
) -> bool {
    if !is_valid(r, c, row, col, dir) {
        return false;
    }

    let positions: Vec<(usize, usize)> = match &dir {
        Direction::Top => (0..4).map(|i| (r - i, c)).collect(),
        Direction::Bottom => (0..4).map(|i| (r + i, c)).collect(),
        Direction::Left => (0..4).map(|i| (r, c - i)).collect(),
        Direction::Right => (0..4).map(|i| (r, c + i)).collect(),
        Direction::TopLeft => (0..4).map(|i| (r - i, c - i)).collect(),
        Direction::TopRight => (0..4).map(|i| (r - i, c + i)).collect(),
        Direction::BottomLeft => (0..4).map(|i| (r + i, c - i)).collect(),
        Direction::BottomRight => (0..4).map(|i| (r + i, c + i)).collect(),
    };

    positions
        .iter()
        .enumerate()
        .all(|(i, &(row, col))| lines[row].as_bytes()[col] == PATTERNS[i])
}

fn is_valid(r: usize, c: usize, row: usize, col: usize, dir: Direction) -> bool {
    match dir {
        Direction::Top => r as i32 - 3 >= 0,
        Direction::Bottom => r + 3 < row,
        Direction::Left => c as i32 - 3 >= 0,
        Direction::Right => c + 3 < col,
        Direction::TopLeft => r as i32 - 3 >= 0 && c as i32 - 3 >= 0,
        Direction::TopRight => r as i32 - 3 >= 0 && c + 3 < col,
        Direction::BottomLeft => r + 3 < row && c as i32 - 3 >= 0,
        Direction::BottomRight => r + 3 < row && c + 3 < col,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(solve(input), 18);
    }
}
