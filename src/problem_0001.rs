// 1. Two Sum: https://leetcode.com/problems/two-sum

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn example_02() {
        let nums = vec![3, 2, 4];
        let target = 6;

        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn example_03() {
        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
