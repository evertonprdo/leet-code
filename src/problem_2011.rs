// 2011. Final Value of Variable After Performing Operations: https://leetcode.com/problems/final-value-of-variable-after-performing-operations

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut ans = 0;
        for op in operations {
            if op.contains('+') {
                ans += 1;
            } else {
                ans -= 1;
            }
        }
        ans
    }
}
pub struct MidPointSolution {}
impl MidPointSolution {
    // Someone's solution
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.into_iter().fold(0, |acc, op| {
            acc + if op.as_bytes()[1] == b'+' { 1 } else { -1 }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = ["--X", "X++", "X++"];
        let output = 1;

        let operations = input.map(|s| s.to_string()).to_vec();
        assert_eq!(Solution::final_value_after_operations(operations), output);

        let operations = input.map(|s| s.to_string()).to_vec();
        assert_eq!(
            MidPointSolution::final_value_after_operations(operations),
            output
        );
    }

    #[test]
    fn example_02() {
        let input = ["++X", "++X", "X++"];
        let output = 3;

        let operations = input.map(|s| s.to_string()).to_vec();
        assert_eq!(Solution::final_value_after_operations(operations), output);

        let operations = input.map(|s| s.to_string()).to_vec();
        assert_eq!(
            MidPointSolution::final_value_after_operations(operations),
            output
        );
    }

    #[test]
    fn example_03() {
        let input = ["X++", "++X", "--X", "X--"];
        let output = 0;

        let operations = input.map(|s| s.to_string()).to_vec();
        assert_eq!(Solution::final_value_after_operations(operations), output);

        let operations = input.map(|s| s.to_string()).to_vec();
        assert_eq!(
            MidPointSolution::final_value_after_operations(operations),
            output
        );
    }
}
