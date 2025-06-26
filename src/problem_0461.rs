// 461. Hamming Distance: https://leetcode.com/problems/hamming-distance

pub struct Solution {}
impl Solution {
    pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
        let mut ans = 0;

        while x > 0 || y > 0 {
            if (x & 1) != (y & 1) {
                ans += 1;
            }

            x >>= 1;
            y >>= 1;
        }

        ans
    }
}

pub struct XorSolution {}
impl XorSolution {
    // https://leetcode.com/problems/hamming-distance/solutions/6652635/master-bit-tricks-to-count-hamming-distance-in-constant-time/
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut dif = x ^ y;
        let mut ans = 0;

        while dif != 0 {
            dif &= dif - 1;
            ans += 1;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let (x, y) = (1, 4);
        let output = 2;

        assert_eq!(Solution::hamming_distance(x, y), output);
        assert_eq!(XorSolution::hamming_distance(x, y), output);
    }

    #[test]
    fn example_02() {
        let (x, y) = (3, 1);
        let output = 1;

        assert_eq!(Solution::hamming_distance(x, y), output);
        assert_eq!(XorSolution::hamming_distance(x, y), output);
    }
}
