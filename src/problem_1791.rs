// 1791. Find Center of Star Graph: https://leetcode.com/problems/find-center-of-star-graph

pub struct Solution {}
impl Solution {
    // Time Complexity: O(1)
    // Space Complexity: O(1)
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let first = &edges.get(0).unwrap();
        let second = &edges.get(1).unwrap();

        let left = first[0];
        if left == second[0] || left == second[1] {
            return left;
        }
        first[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = [[1, 2], [2, 3], [4, 2]];
        let output = 2;

        let edges: Vec<Vec<i32>> = input.map(|x| x.to_vec()).to_vec();
        assert_eq!(Solution::find_center(edges), output);
    }

    #[test]
    fn example_02() {
        let input = [[1, 2], [5, 1], [1, 3], [1, 4]];
        let output = 1;

        let edges: Vec<Vec<i32>> = input.map(|x| x.to_vec()).to_vec();
        assert_eq!(Solution::find_center(edges), output);
    }
}
