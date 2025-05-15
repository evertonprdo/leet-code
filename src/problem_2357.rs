// 2357. Make Array Zero by Subtracting Equal Amounts: https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut bitset: [u64; 2] = [0; 2];
        let mut unique_non_zeros = 0;

        for n in nums {
            if n != 0 {
                let n = n as usize;
                let (w, b) = (n >> 6, n % 64);

                if (bitset[w] & (1 << b)) == 0 {
                    bitset[w] |= 1 << b;
                    unique_non_zeros += 1;
                }
            }
        }

        unique_non_zeros
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = [1, 5, 0, 3, 5];
        let output = 3;

        assert_eq!(Solution::minimum_operations(nums.to_vec()), output);
    }

    #[test]
    fn example_02() {
        let nums = [0];
        let output = 0;

        assert_eq!(Solution::minimum_operations(nums.to_vec()), output);
    }

    #[test]
    fn example_03() {
        let nums = [7, 28, 34, 76];
        let output = 4;

        assert_eq!(Solution::minimum_operations(nums.to_vec()), output);
    }
}
