pub mod part1;
pub mod part2;

pub fn combination(k: i32, source: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut results = vec![];
    let mut cur = vec![];
    recur(k, 0, &mut cur, &mut results, &source);
    results
}

fn recur(
    k: i32,
    step: i32,
    cur: &mut Vec<(usize, usize)>,
    results: &mut Vec<Vec<(usize, usize)>>,
    source: &Vec<(usize, usize)>,
) {
    if cur.len() == k as usize {
        results.push(cur.clone());
        return;
    }

    for i in step..source.len() as i32 {
        cur.push(source[i as usize]);
        recur(k, i + 1, cur, results, source);
        cur.pop();
    }
}
