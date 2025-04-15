// 69. Sqrt(x): https://leetcode.com/problems/sqrtx/

pub struct Solution {}
impl Solution {
    // Time complexity: O(Sqrt(N))
    // Space complexity: O(1)
    pub fn my_sqrt(x: i32) -> i32 {
        let square: u32 = x as u32;
        let mut i: u32 = 1;

        loop {
            let sqrt = i * i;
            if sqrt == square {
                return i as i32;
            }

            if sqrt > square {
                return (i - 1) as i32;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let x = 4;
        assert_eq!(Solution::my_sqrt(x), 2);
    }

    #[test]
    fn example_02() {
        let x = 8;
        assert_eq!(Solution::my_sqrt(x), 2);
    }
}
