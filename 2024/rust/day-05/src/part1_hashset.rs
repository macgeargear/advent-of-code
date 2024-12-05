use crate::parse_input;

pub fn solve(input: &str) -> i32 {
    let (rules, page_orders) = parse_input(input);
    let total: i32 = page_orders
        .into_iter()
        .filter_map(|order| {
            if order.is_sorted_by(|l, r| rules.contains(&(*l, *r))) {
                Some(order[order.len() / 2])
            } else {
                None
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
        assert_eq!(solve(input), 143);
    }
}
