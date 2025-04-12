// 941. Valid Mountain Array: https://leetcode.com/problems/valid-mountain-array

fn main() {
    let arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    println!("Solution: {}", Solution::valid_mountain_array(arr.clone()));
    println!(
        "Climb Mountain Solution: {}",
        ClimbMountain::valid_mountain_array(arr.clone())
    );
    println!(
        "Spaghetti Solution: {}",
        SpaghettiSolution::valid_mountain_array(arr.clone())
    );
}

// First attempt
// Temporal Complexity: O(n)
// Space Complexity: O(1)
struct Solution {}
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
struct ClimbMountain {}
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
struct SpaghettiSolution {}
impl SpaghettiSolution {
    #[rustfmt::skip]
    fn valid_mountain_array(A: Vec<i32>) -> bool {
        let n = A.len(); let mut i = 0; let mut j = n - 1;
        while i + 1 < n && A[i] < A[i + 1] { i += 1; }
        while j > 0 && A[j - 1] > A[j] { j -= 1; }
        i > 0 && i == j && j < n - 1
    }
}
