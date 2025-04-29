// 338. Counting Bits: https://leetcode.com/problems/counting-bits

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n log n)
    // Space Complexity: O(n)
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity((n + 1) as usize);
        for mut i in 0..=n {
            let mut count = 0;
            while i > 0 {
                if i & 1 == 1 {
                    count += 1;
                }
                i >>= 1;
            }
            ans.push(count);
        }
        ans
    }
}

pub struct BitwiseSolution {}
impl BitwiseSolution {
    // https://leetcode.com/problems/counting-bits/solutions/6635451/unlock-the-bitwise-recursion-trick-to-count-set-bits-in-o-n-time
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans: Vec<i32> = vec![0; n + 1];
        for i in 0..=n {
            let x = i as i32;
            ans[i] = ans[i >> 1] + (x & 1);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 2;
        let output = vec![0, 1, 1];

        assert_eq!(Solution::count_bits(n), output);
        assert_eq!(BitwiseSolution::count_bits(n), output);
    }

    #[test]
    fn example_02() {
        let n = 5;
        let output = vec![0, 1, 1, 2, 1, 2];

        assert_eq!(Solution::count_bits(n), output);
        assert_eq!(BitwiseSolution::count_bits(n), output);
    }
}
