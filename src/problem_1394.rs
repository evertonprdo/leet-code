// 1394. Find Lucky Integer in an Array: https://leetcode.com/problems/find-lucky-integer-in-an-array

pub struct Solution;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for n in arr {
            map.entry(n).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut ans = -1;
        for (k, v) in map {
            if k == v {
                ans = ans.max(k)
            };
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let arr = [2, 2, 3, 4];
        let output = 2;

        assert_eq!(Solution::find_lucky(arr.to_vec()), output);
    }

    #[test]
    fn example_02() {
        let arr = [1, 2, 2, 3, 3, 3];
        let output = 3;

        assert_eq!(Solution::find_lucky(arr.to_vec()), output);
    }

    #[test]
    fn example_03() {
        let arr = [2, 2, 2, 3, 3];
        let output = -1;

        assert_eq!(Solution::find_lucky(arr.to_vec()), output);
    }
}
