// 66. Plus One: https://leetcode.com/problems/plus-one

pub struct Solution {}
impl Solution {
    // Time complexity: O(n)
    // Space complexity: O(1)
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut one_was_added = false;

        for d in digits.iter_mut().rev() {
            if *d == 9 {
                *d = 0;
            } else {
                *d = *d + 1;
                one_was_added = true;

                break;
            }
        }

        if one_was_added {
            return digits;
        }

        digits.insert(0, 1);
        return digits;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let digits = vec![1, 2, 3];
        assert_eq!(Solution::plus_one(digits), vec![1, 2, 4]);
    }

    #[test]
    fn example_02() {
        let digits = vec![4, 3, 2, 1];
        assert_eq!(Solution::plus_one(digits), vec![4, 3, 2, 2]);
    }

    #[test]
    fn example_03() {
        let digits = vec![9];
        assert_eq!(Solution::plus_one(digits), vec![1, 0]);
    }
}
