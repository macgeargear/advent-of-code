/*
0 -> 1
even # of digits -> e.g. 1000 -> 10 0
else -> n * 2024
*/

const TIMES: usize = 25;

#[derive(Debug)]
struct Stone {
    value: u64,
}

impl Stone {
    fn new(value: u64) -> Self {
        Self { value }
    }

    fn blink(&self) -> Vec<u64> {
        if self.value == 0 {
            return vec![1];
        }

        if self.value.to_string().len() % 2 == 0 {
            return Self::split(&self);
        }

        vec![self.value * 2024]
    }

    fn split(&self) -> Vec<u64> {
        // println!("value: {:?}", self.value);
        let value = self.value.to_string();
        let mid = value.len() / 2;
        // println!(
        //     "splitted: {:?}",
        //     vec![
        //         value[..mid].parse::<u64>().unwrap(),
        //         value[mid..].parse::<u64>().unwrap()
        //     ]
        // );
        vec![value[..mid].parse().unwrap(), value[mid..].parse().unwrap()]
    }
}

pub fn solve(input: &str) -> i32 {
    let input: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut cur_stone: Vec<u64> = input.clone();

    for i in 0..TIMES {
        let mut new_stones: Vec<u64> = Vec::new();
        for stone in cur_stone.iter() {
            let stone = Stone::new(*stone);
            new_stones.extend(stone.blink());
        }
        println!("{}: {:?}", i + 1, new_stones);
        cur_stone = new_stones;
    }

    cur_stone.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "125 17";
        assert_eq!(solve(input), 55312);
    }
}
