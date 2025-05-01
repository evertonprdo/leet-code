// 26. Remove Duplicates from Sorted Array: https://leetcode.com/problems/remove-duplicates-from-sorted-array

pub struct Solution {}
impl Solution {
    // Inspired by the “Snowball” solution (see problem_0283.rs)
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    //
    // The `l` pointer trails `r` until it encounters the first duplicate.
    // At that point, `l` marks the position where the next unique element should be placed.
    //
    // In a no-duplicate scenario, `r - l == 0` and nothing is changed.
    // Once a duplicate is found, `r` continues to advance, so `r - l > 0`.
    // When a new unique value appears, it’s copied into `nums[l]`, and `l` is incremented.
    //
    // Any values “behind” `l` are overwritten in-place (not swapped), since they’re no longer needed.
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut l = 1;

        for r in l..nums.len() {
            if nums[r] != nums[r - 1] {
                nums[l] = nums[r];
                l += 1;
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
        let input = vec![1, 1, 2];
        let output = 2;
        let expected = vec![1, 2];

        let mut nums = input.clone();
        assert_eq!(Solution::remove_duplicates(&mut nums), output);
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }
    }

    #[test]
    fn example_02() {
        let input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let output = 5;
        let expected = vec![0, 1, 2, 3, 4];

        let mut nums = input.clone();
        assert_eq!(Solution::remove_duplicates(&mut nums), output);
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }
    }
}
