pub mod part1_hashset;
pub mod part1_stupid;
pub mod part2;

use std::collections::HashSet;

pub fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: HashSet<(i32, i32)> = HashSet::new();
    let mut page_orders: Vec<Vec<i32>> = Vec::new();
    let mut empty_line = false;

    for line in input.lines().map(str::trim) {
        if line.is_empty() {
            empty_line = true;
            continue;
        }
        let sep = if empty_line { "," } else { "|" };
        let parsed_line: Vec<i32> = line.split(sep).map(|x| x.parse::<i32>().unwrap()).collect();
        if empty_line {
            page_orders.push(parsed_line);
        } else {
            rules.insert((parsed_line[0], parsed_line[1]));
        }
    }

    (rules, page_orders)
}
