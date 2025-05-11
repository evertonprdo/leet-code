// 1550. Three Consecutive Odds: https://leetcode.com/problems/three-consecutive-odds

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() > 2 {
            let f = |x| x & 1 == 1;

            for i in 0..arr.len() - 2 {
                if f(arr[i]) && f(arr[i + 1]) && f(arr[i + 2]) {
                    return true;
                }
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
        let arr = [2, 6, 4, 1];

        assert!(!Solution::three_consecutive_odds(arr.to_vec()));
    }

    #[test]
    fn example_02() {
        let arr = [1, 2, 34, 3, 4, 5, 7, 23, 12];

        assert!(Solution::three_consecutive_odds(arr.to_vec()));
    }

    #[test]
    fn example_03() {
        let arr = [1];

        assert!(!Solution::three_consecutive_odds(arr.to_vec()));
    }
}
