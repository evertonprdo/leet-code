// 367. Valid Perfect Square: https://leetcode.com/problems/valid-perfect-square

use std::cmp::Ordering;

pub struct Solution {}
impl Solution {
    // First attempt
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }
        let mut sqrt = num / 2;
        while sqrt > 1 {
            if sqrt * sqrt == num {
                return true;
            }

            sqrt -= 1;
        }
        false
    }
}

pub struct BinarySolution {}
impl BinarySolution {
    // https://leetcode.com/problems/valid-perfect-square/solutions/1954741/rust-solution
    pub fn is_perfect_square(num: i32) -> bool {
        let mut low = 1;
        let mut high = num;

        while low <= high {
            let mid = low + (high - low) / 2;

            match mid.checked_mul(mid) {
                Some(sqrt) => match sqrt.cmp(&num) {
                    Ordering::Equal => return true,
                    Ordering::Less => low = mid + 1,
                    Ordering::Greater => high = mid - 1,
                },
                None => high = mid - 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let num = 16;
        assert!(Solution::is_perfect_square(num));
        assert!(BinarySolution::is_perfect_square(num));
    }

    #[test]
    fn example_02() {
        let num = 14;
        assert!(!Solution::is_perfect_square(num));
        assert!(!BinarySolution::is_perfect_square(num));
    }

    #[test]
    fn example_03() {
        let num = 1;
        assert!(Solution::is_perfect_square(num));
        assert!(BinarySolution::is_perfect_square(num));
    }
}
