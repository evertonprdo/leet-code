// 217. Contains Duplicate: https://leetcode.com/problems/contains-duplicate

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = BitSet::new();

        for n in nums {
            if set.get(n) {
                return true;
            } else {
                set.set(n);
            }
        }

        false
    }
}

// https://www.linkedin.com/feed/update/urn:li:activity:7328400161590702082/
const ABS_MAX: usize = 1_000_000_000; // Range is +/- 10.pow(9)
const N_VALS: usize = ABS_MAX * 2 + 1; // Number of possible values.

const N_BITS: usize = 64;
const OFFSET: i32 = 1_000_000_000;

pub struct BitSet {
    bits: Vec<u64>,
}
impl BitSet {
    pub fn new() -> Self {
        let num_words = (N_VALS + N_BITS - 1) / N_BITS;

        BitSet {
            bits: vec![0; num_words],
        }
    }

    fn word_bit_of(index: i32) -> (usize, usize) {
        let index = (index + OFFSET) as usize;

        let word = index / N_BITS;
        let bit = index % N_BITS;

        (word, bit)
    }

    pub fn set(&mut self, index: i32) {
        let (w, b) = Self::word_bit_of(index);

        self.bits[w] |= 1 << b;
    }

    pub fn get(&self, index: i32) -> bool {
        let (w, b) = Self::word_bit_of(index);

        (self.bits[w] & (1 << b)) != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = [1, 2, 3, 1];
        assert!(Solution::contains_duplicate(nums.to_vec()));
    }

    #[test]
    fn example_02() {
        let nums = [1, 2, 3, 4];
        assert!(!Solution::contains_duplicate(nums.to_vec()));
    }

    #[test]
    fn example_03() {
        let nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(Solution::contains_duplicate(nums.to_vec()));
    }

    #[test]
    fn example_04() {
        let nums = [1, 5, -2, -4, 0];
        assert!(!Solution::contains_duplicate(nums.to_vec()));
    }
}
