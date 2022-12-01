use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn part01(input: &str) -> i64 {
    collect_capacity(input, 1)
}

pub fn part02(input: &str) -> i64 {
    collect_capacity(input, 3)
}

fn collect_capacity(input: &str, k: usize) -> i64 {
    // we use Reverse to keep a min heap of the largest k elements
    let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut curr = 0i64;

    let mut heap_push = |n: i64| {
        if heap.len() < k || n > heap.peek().unwrap().0 {
            heap.push(Reverse(n));

            // trim heap to desired length
            if heap.len() > k {
                heap.pop();
            }
        }
    };

    for line in input.lines() {
        match line.is_empty() {
            true => {
                heap_push(curr);
                curr = 0;
            }
            false => curr += line.parse::<i64>().unwrap(),
        }
    }
    // add the final line since it doesn't have a trailing newline
    heap_push(curr);

    heap.iter().map(|r| r.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let ans = part01(include_str!("../inputs/1.sample.txt"));
        assert_eq!(24000, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(include_str!("../inputs/1.input.txt"));
        assert_eq!(69501, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(include_str!("../inputs/1.sample.txt"));
        assert_eq!(45000, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(include_str!("../inputs/1.input.txt"));
        assert_eq!(202346, ans);
    }
}
