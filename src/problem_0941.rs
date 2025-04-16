// 941. Valid Mountain Array: https://leetcode.com/problems/valid-mountain-array

// First attempt
// Temporal Complexity: O(n)
// Space Complexity: O(1)
pub struct Solution {}
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 || arr[0] > arr[1] {
            return false;
        }

        let mut increasing = true;
        let mut i = 0;

        loop {
            match arr.get(i + 1) {
                None => return !increasing,
                Some(next) => {
                    let curr = arr[i];

                    if curr == *next || (!increasing && curr < *next) {
                        return false;
                    }
                    if curr > *next {
                        increasing = false;
                    }
                }
            }
            i += 1;
        }
    }
}

// Climb Mountain Solution
// https://leetcode.com/problems/valid-mountain-array/solutions/1717377/java-c-python-easy-to-go-through-solution-explanation/
pub struct ClimbMountain {}
impl ClimbMountain {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 || arr[0] > arr[1] {
            return false;
        }

        let mut i = 0;
        loop {
            match arr.get(i + 1) {
                None => return false,
                Some(val) => {
                    if arr[i] >= *val {
                        break;
                    }
                }
            }
            i += 1;
        }

        let mut j = arr.len() - 1;
        loop {
            let (k, over) = j.overflowing_sub(1);
            if over || arr[j] >= arr[k] {
                break;
            }
            j = k;
        }

        return i == j;
    }
}

// https://leetcode.com/problems/valid-mountain-array/solutions/194900/c-java-python-climb-mountain/
pub struct SpaghettiSolution {}
impl SpaghettiSolution {
    #[rustfmt::skip]
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len(); let mut i = 0; let mut j = n - 1;
        while i + 1 < n && arr[i] < arr[i + 1] { i += 1; }
        while j > 0 && arr[j - 1] > arr[j] { j -= 1; }
        i > 0 && i == j && j < n - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let arr = vec![2, 1];

        assert!(!Solution::valid_mountain_array(arr.clone()));
        assert!(!ClimbMountain::valid_mountain_array(arr.clone()));
        assert!(!SpaghettiSolution::valid_mountain_array(arr.clone()));
    }

    #[test]
    fn example_02() {
        let arr = vec![3, 5, 5];

        assert!(!Solution::valid_mountain_array(arr.clone()));
        assert!(!ClimbMountain::valid_mountain_array(arr.clone()));
        assert!(!SpaghettiSolution::valid_mountain_array(arr.clone()));
    }

    #[test]
    fn example_03() {
        let arr = vec![0, 3, 2, 1];

        assert!(Solution::valid_mountain_array(arr.clone()));
        assert!(ClimbMountain::valid_mountain_array(arr.clone()));
        assert!(SpaghettiSolution::valid_mountain_array(arr.clone()));
    }
}
