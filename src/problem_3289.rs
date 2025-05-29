// 3289. The Two Sneaky Numbers of Digitville: https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        // It's not good to assume that the target is 64 bits;
        // But for the sake of simplicity, I will ignore the 32 bits case;
        const BITS: usize = 64;

        let mut bitset = [0_usize; 2];
        let mut ans = Vec::with_capacity(2);

        for num in nums {
            let n = num as usize;
            let word = n / BITS;
            let bit = n % BITS;

            if bitset[word] & (1 << bit) == 0 {
                bitset[word] |= 1 << bit;
            } else {
                ans.push(num);

                if ans.len() == 2 {
                    break;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_solution<F, const N: usize>(solution: F, args: [i32; N], expected: [i32; 2])
    where
        F: Fn(Vec<i32>) -> Vec<i32>,
    {
        let nums = args.to_vec();
        let expected = expected.to_vec();

        let output = solution(nums);

        assert_eq!(output.len(), expected.len());

        assert!(output.contains(&expected[0]));
        assert!(output.contains(&expected[1]));
    }

    #[test]
    fn example_01() {
        let nums = [0, 1, 1, 0];
        let output = [0, 1];

        assert_solution(Solution::get_sneaky_numbers, nums, output);
    }

    #[test]
    fn example_02() {
        let nums = [0, 3, 2, 1, 3, 2];
        let output = [2, 3];

        assert_solution(Solution::get_sneaky_numbers, nums, output);
    }

    #[test]
    fn example_03() {
        let nums = [7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2];
        let output = [4, 5];

        assert_solution(Solution::get_sneaky_numbers, nums, output);
    }
}
