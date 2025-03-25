// 1. Two Sum: https://leetcode.com/problems/two-sum

use std::collections::HashMap;

fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;

    println!("{:?}", two_sum(nums, target));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut iter = nums.iter();
    let mut i = 0;

    loop {
        let n = iter.next().unwrap();
        if let Some(j) = map.get(n) {
            return vec![*j, i];
        }
        map.insert(target - n, i);
        i += 1;
    }
}

// I already solved this problem in typescript before, so I tried to solve it of the top of my head.
// And I realized that by doing this, I ended up solving it in a different way.

/* First attempt

    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut iter = nums.iter();
        let mut i = 0;

        while let Some(n) = iter.next() {
            if let Some(x) = map.get(&n) {
                return vec![*x, i];
            }

            map.insert(target - n, i);
            i += 1;
        }

        vec![]
    }
*/
/* Semantic try:

    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            if let Some(j) = map.get(n) {
                return vec![*j as i32, i as i32];
            }

            map.insert(target - n, i);
        }

        vec![]
    }
*/
