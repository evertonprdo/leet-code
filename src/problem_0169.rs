// 169. Majority Element: https://leetcode.com/problems/majority-element

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let majority_len = nums.len() / 2;

        let mut iter = nums.into_iter();
        loop {
            let n = iter.next().unwrap();
            map.entry(n).and_modify(|c| *c += 1).or_insert(1);

            if map[&n] > majority_len {
                return n;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = [3, 2, 3];
        let output = 3;

        assert_eq!(Solution::majority_element(nums.to_vec()), output);
    }

    #[test]
    fn example_02() {
        let nums = [2, 2, 1, 1, 1, 2, 2];
        let output = 2;

        assert_eq!(Solution::majority_element(nums.to_vec()), output);
    }
}
