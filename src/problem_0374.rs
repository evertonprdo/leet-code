// 374. Guess Number Higher or Lower: https://leetcode.com/problems/guess-number-higher-or-lower
use std::cmp::Ordering;

pub struct Solution {
    pub pick: i32,
}
impl Solution {
    // Time Complexity: O(log n)
    // Space Complexity: O(1)
    pub fn guess_number(&self, n: i32) -> i32 {
        let mut low = 1;
        let mut high = n;

        while low < high {
            let mid = low + (high - low >> 1);
            match self.guess(mid) {
                0 => return mid,
                1 => low = mid + 1,
                _ => high = mid,
            }
        }

        n
    }

    // Forward declaration of guess API.
    // @param  num   your guess
    // @return 	     -1 if num is higher than the picked number
    //			      1 if num is lower than the picked number
    //               otherwise return 0
    //
    fn guess(&self, num: i32) -> i32 {
        match num.cmp(&self.pick) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1,
        }
    }
}
pub struct LoopSolution {
    pub pick: i32,
}
impl LoopSolution {
    // Time Complexity: O(log n)
    // Space Complexity: O(1)
    pub fn guess_number(&self, n: i32) -> i32 {
        let mut low = 1;
        let mut high = n;

        loop {
            let mid = low + (high - low >> 1);
            match self.guess(mid) {
                0 => return mid,
                1 => low = mid + 1,
                _ => high = mid - 1,
            }
        }
    }

    // Forward declaration of guess API.
    // @param  num   your guess
    // @return 	     -1 if num is higher than the picked number
    //			      1 if num is lower than the picked number
    //               otherwise return 0
    //
    fn guess(&self, num: i32) -> i32 {
        match num.cmp(&self.pick) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input = 10;
        let pick = 6;

        let solution = Solution { pick };
        assert_eq!(solution.guess_number(input), pick);

        let solution = LoopSolution { pick };
        assert_eq!(solution.guess_number(input), pick);
    }

    #[test]
    fn example_02() {
        let input = 1;
        let pick = 1;

        let solution = Solution { pick };
        assert_eq!(solution.guess_number(input), pick);

        let solution = LoopSolution { pick };
        assert_eq!(solution.guess_number(input), pick);
    }

    #[test]
    fn example_03() {
        let input = 2;
        let pick = 1;

        let solution = Solution { pick };
        assert_eq!(solution.guess_number(input), pick);

        let solution = LoopSolution { pick };
        assert_eq!(solution.guess_number(input), pick);
    }
}
