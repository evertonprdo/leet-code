// 26. Remove Duplicates from Sorted Array: https://leetcode.com/problems/remove-duplicates-from-sorted-array

pub struct Solution {}
impl Solution {
    // Inspired in the Snowball Solution: problem_0283.rs
    // Time Complexity: O(n)
    // Space Complexity: O(1)

    // `l` follows `r` until a duplicate pair appears,
    // so `l` stores the index of the first duplicate value in the array.

    // In other words, when l - r == 0 nothing changes.
    // When a duplicate pair appears, r increments,
    // creating an offset between l and r, making r - l > 0.
    // This causes arr[l] to receive the next non-duplicate value.

    // After an offset occurs, all non-duplicate values ​​are copied accordingly,
    // because now r - l > 0 is always true, a duplicate value just increments that offset.

    // The value of left is overwritten and not swapped,
    // because those values ​​are no longer important.
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
