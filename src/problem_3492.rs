// 3492. Maximum Containers on a Ship: https://leetcode.com/problems/maximum-containers-on-a-ship

use std::cmp::min;

pub struct Solution {}
impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        let cells = n * n;
        let n_weight = max_weight / w;

        min(cells, n_weight)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let n = 2;
        let w = 3;
        let max_weight = 15;
        let output = 4;

        assert_eq!(Solution::max_containers(n, w, max_weight), output);
    }

    #[test]
    fn example_02() {
        let n = 3;
        let w = 5;
        let max_weight = 20;
        let output = 4;

        assert_eq!(Solution::max_containers(n, w, max_weight), output);
    }
}
