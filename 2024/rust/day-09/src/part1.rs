use std::usize;

pub fn solve(input: &str) -> u64 {
    let input = input.trim();
    let mut files = Vec::new();
    let mut cnt = 0;
    for (i, c) in input.chars().enumerate() {
        let n = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.extend(vec![cnt; n]);
            cnt += 1;
        } else {
            files.extend(vec![usize::MAX; n]);
        }
    }
    println!("{}", input.len());
    let (mut l, mut r) = (0 as usize, (files.len() - 1));
    while l <= r {
        if files[l] == usize::MAX && files[r] != usize::MAX {
            files[l] = files[r];
            files[r] = usize::MAX;
            l += 1;
            r -= 1;
        } else if files[l] != usize::MAX {
            l += 1;
        } else {
            r -= 1;
        }
    }

    let mut total: u64 = 0;
    for (i, &file) in files.iter().enumerate() {
        if file == usize::MAX {
            break;
        }
        let i = i as u64;
        total += (file as u64) * (i as u64);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "2333133121414131402";
        assert_eq!(solve(input), 1928);
    }

    #[test]
    fn test_solve2() {
        let input = "12345";
        assert_eq!(solve(input), 60);
    }

    #[test]
    fn test_solve3() {
        let input = "90909";
        assert_eq!(solve(input), 27);
    }
}
