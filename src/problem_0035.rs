// 35. Search Insert Position: https://leetcode.com/problems/search-insert-position

pub struct Solution {}
impl Solution {
    // Binary Search
    // Time complexity: O(log n)
    // Space complexity: O(1)
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let i = l + (r - l) / 2;

            match target.cmp(&nums[i]) {
                std::cmp::Ordering::Equal => return i as i32,

                std::cmp::Ordering::Greater => l = i + 1,
                std::cmp::Ordering::Less => r = i,
            }
        }

        l as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;

        assert_eq!(Solution::search_insert(nums, target), 2);
    }

    #[test]
    fn example_02() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;

        assert_eq!(Solution::search_insert(nums, target), 1);
    }

    #[test]
    fn example_03() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;

        assert_eq!(Solution::search_insert(nums, target), 4);
    }
}
