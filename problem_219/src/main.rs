// 219. Contains Duplicate II: https://leetcode.com/problems/contains-duplicate-ii
use std::collections::HashMap;

fn main() {
    // let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 9]; // [1,2,3,1]
    let nums = [1, 2, 3, 1, 2, 3];

    let k = 2;

    println!("{}", contains_nearby_duplicate(nums.to_vec(), k));
    println!("{}", Solution::contains_nearby_duplicate(nums.to_vec(), k))
}

// Based on the 0ms solution:
fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map: HashMap<&i32, usize> = HashMap::with_capacity(nums.len());
    let k = k as usize;

    for (i, n) in nums.iter().enumerate() {
        match map.get(n) {
            Some(prev) if i - prev <= k => return true,
            _ => map.insert(n, i),
        };
    }

    false
}

// https://leetcode.com/problems/contains-duplicate-ii/solutions/1627864/rust-replacing-hashmap-with-a-bit-set-faster-than-100/?envType=problem-list-v2&envId=sliding-window
const ABS_MAX: usize = 1_000_000_000; // Range is +/- 10.pow(9)
const N_VALS: usize = ABS_MAX * 2 + 1; // Number of possible values.

struct Solution {}
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 || nums.len() < 2 {
            return false;
        }

        let n = nums.len();
        let k = (k as usize).min(n - 1);
        let mut bits = BitSet::new(N_VALS);

        for i in 0..=k {
            let num = nums[i] as usize + ABS_MAX;
            if bits.get(num) {
                return true;
            }
            bits.set(num, true);
        }
        for i in k + 1..n {
            let num_out = nums[i - k - 1] as usize + ABS_MAX;
            let num_in = nums[i] as usize + ABS_MAX;

            bits.set(num_out, false);

            if bits.get(num_in) {
                return true;
            }
            bits.set(num_in, true);
        }

        false
    }
}

const N_BITS: usize = 64;
type BlockType = u64;

struct BitSet {
    bits: Vec<BlockType>,
}

impl BitSet {
    fn new(n_vals: usize) -> Self {
        Self {
            bits: vec![0; (n_vals + N_BITS - 1) / N_BITS],
        }
    }

    fn get(&self, idx: usize) -> bool {
        (self.bits[idx / N_BITS] >> idx % N_BITS) & 1 == 1
    }

    fn set(&mut self, idx: usize, val: bool) {
        if val {
            self.bits[idx / N_BITS] |= 1 << idx % N_BITS;
        } else {
            self.bits[idx / N_BITS] &= !(1 << idx % N_BITS);
        }
    }
}
