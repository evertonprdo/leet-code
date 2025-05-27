// 771. Jewels and Stones: https://leetcode.com/problems/jewels-and-stones

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n + m)
    // Space Complexity: O(m)
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut map = HashMap::new();
        for c in jewels.chars() {
            map.insert(c, 0);
        }

        for c in stones.chars() {
            if let Some(jew) = map.get_mut(&c) {
                *jew += 1;
            }
        }

        map.values().sum()
    }
}

pub struct BitSetSolution {}
impl BitSetSolution {
    // Time Complexity: O(n + m)
    // Space Complexity: O(1)
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut set = 0_u64;

        for b in jewels.into_bytes() {
            let bit = (b - 65) % 64;
            set |= 1 << bit;
        }

        let mut ans = 0;
        for b in stones.into_bytes() {
            let bit = (b - 65) % 64;

            if set & (1 << bit) != 0 {
                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn asset_solution<F>(arg1: &str, arg2: &str, expected: i32, solution: F)
    where
        F: Fn(String, String) -> i32,
    {
        let jewels = arg1.to_string();
        let stones = arg2.to_string();

        assert_eq!(solution(jewels, stones), expected);
    }

    #[test]
    fn example_01() {
        let jewels = "aA";
        let stones = "aAAbbbb";
        let output = 3;

        asset_solution(jewels, stones, output, Solution::num_jewels_in_stones);
        asset_solution(jewels, stones, output, BitSetSolution::num_jewels_in_stones);
    }

    #[test]
    fn example_02() {
        let jewels = "z";
        let stones = "ZZ";
        let output = 0;

        asset_solution(jewels, stones, output, Solution::num_jewels_in_stones);
        asset_solution(jewels, stones, output, BitSetSolution::num_jewels_in_stones);
    }
}
