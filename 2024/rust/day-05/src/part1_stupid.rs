use std::collections::HashMap;

pub fn solve(input: &str) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
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
            let (l, r) = parsed_line.split_first().unwrap();
            map.entry(*l).or_insert_with(Vec::new).extend(r);
        }
    }

    println!("{:#?}", map);

    let total: i32 = page_orders
        .into_iter()
        .filter_map(|order| {
            if order.windows(2).all(|nums| {
                let (l, r) = (nums[0], nums[1]);
                if let Some(v) = map.get(&l) {
                    v.contains(&r)
                } else {
                    false
                }
            }) {
                Some(order[order.len() / 2 as usize])
            } else {
                None
            }
        })
        .sum::<i32>();

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
