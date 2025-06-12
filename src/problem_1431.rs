// 1431. Kids With the Greatest Number of Candies: https://leetcode.com/problems/kids-with-the-greatest-number-of-candies

pub struct Solution {}
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        candies.iter().map(|n| n + extra_candies >= max).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert<F, const N: usize>(solution: F, input: ([i32; N], i32), expect: [bool; N])
    where
        F: Fn(Vec<i32>, i32) -> Vec<bool>,
    {
        let candies = input.0.to_vec();
        let extra_candies = input.1;

        assert_eq!(solution(candies, extra_candies), expect.to_vec());
    }

    #[test]
    fn example_01() {
        let input = ([2, 3, 5, 1, 3], 3);
        let output = [true, true, true, false, true];

        assert(Solution::kids_with_candies, input, output);
    }

    #[test]
    fn example_02() {
        let input = ([4, 2, 1, 1, 2], 1);
        let output = [true, false, false, false, false];

        assert(Solution::kids_with_candies, input, output);
    }

    #[test]
    fn example_03() {
        let input = ([12, 1, 12], 10);
        let output = [true, false, true];

        assert(Solution::kids_with_candies, input, output);
    }
}
