use std::cmp::Ordering;
use std::collections::HashSet;

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
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

pub fn solve(input: &str) -> i32 {
    let (rules, page_orders) = parse_input(input);
    let total: i32 = page_orders
        .into_iter()
        .filter_map(|mut order| {
            if order.is_sorted_by(|l, r| rules.contains(&(*l, *r))) {
                None
            } else {
                order.sort_by(|l, r| {
                    if rules.contains(&(*l, *r)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                Some(order[order.len() / 2])
            }
        })
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(solve(input), 123);
    }
}
