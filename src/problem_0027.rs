// 27. Remove Element: https://leetcode.com/problems/remove-element

pub struct Solution {}
impl Solution {
    // Same as `problem_0026.rs`
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut l = 0;

        for r in l..nums.len() {
            if nums[r] != val {
                nums[l] = nums[r];
                l += 1;
            }
        }

        l as i32
    }
}

pub struct SolutionSwap {}
impl SolutionSwap {
    // GPT tip
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            if nums[l] == val {
                r -= 1;
                nums.swap(l, r);
            } else {
                l += 1;
            }
        }

        l as i32
    }
}

pub struct SolutionMem {}
impl SolutionMem {
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut tmp = Vec::new();
        let mut k = 0;

        for n in std::mem::take(nums) {
            if n != val {
                tmp.push(n);
                k += 1;
            }
        }
        *nums = tmp;
        k
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = vec![3, 2, 2, 3];
        let val = 3;

        let k = 2;
        let expected = vec![2, 2];

        let mut nums = input.clone();
        assert_eq!(Solution::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionMem::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionSwap::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }
    }

    #[test]
    fn example_02() {
        let input = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;

        let k = 5;
        let expected = vec![0, 0, 1, 3, 4];

        let mut nums = input.clone();
        assert_eq!(Solution::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionMem::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionSwap::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }
    }

    #[test]
    fn example_03() {
        let input = vec![0, 4, 4, 0, 4, 4, 4, 0, 2];
        let val = 4;

        let k = 4;
        let expected = vec![];

        let mut nums = input.clone();
        assert_eq!(Solution::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionMem::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionSwap::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }
    }

    #[test]
    fn example_04() {
        let input = vec![3, 3];
        let val = 3;

        let k = 0;
        let expected = vec![];

        let mut nums = input.clone();
        assert_eq!(Solution::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionMem::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionSwap::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }
    }

    #[test]
    fn example_05() {
        let input = vec![2, 2, 3];
        let val = 2;

        let k = 1;
        let expected = vec![3];

        let mut nums = input.clone();
        assert_eq!(Solution::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionMem::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }

        let mut nums = input.clone();
        assert_eq!(SolutionSwap::remove_element(&mut nums, val), k as i32);

        nums[..k].sort();
        for i in 0..expected.len() {
            assert_eq!(nums[i], expected[i]);
        }
    }
}
