// 1470. Shuffle the Array: https://leetcode.com/problems/shuffle-the-array

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = Vec::with_capacity(nums.len());

        for i in 0..n {
            ans.push(nums[i]);
            ans.push(nums[i + n]);
        }

        ans
    }
}

pub struct InPlaceSolution {}
impl InPlaceSolution {
    // https://leetcode.com/problems/shuffle-the-array/solutions/1314638/java-c-simple-o-1-space-in-place-100-faster-efficient-explained/
    pub fn shuffle(mut nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let len = nums.len();

        for i in n..len {
            nums[i] = (nums[i] << 10) | nums[i - n]
        }

        for (i, k) in (n..len).zip((0..len).step_by(2)) {
            nums[k] = nums[i] & 1023;
            nums[k + 1] = nums[i] >> 10;
        }

        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = [2, 5, 1, 3, 4, 7];
        let output = [2, 3, 5, 4, 1, 7];
        let n = 3;

        assert_eq!(Solution::shuffle(nums.to_vec(), n), output.to_vec());
        assert_eq!(InPlaceSolution::shuffle(nums.to_vec(), n), output.to_vec());
    }

    #[test]
    fn example_02() {
        let nums = [1, 2, 3, 4, 4, 3, 2, 1];
        let output = [1, 4, 2, 3, 3, 2, 4, 1];
        let n = 4;

        assert_eq!(Solution::shuffle(nums.to_vec(), n), output.to_vec());
        assert_eq!(InPlaceSolution::shuffle(nums.to_vec(), n), output.to_vec());
    }

    #[test]
    fn example_03() {
        let nums = [1, 1, 2, 2];
        let output = [1, 2, 1, 2];
        let n = 2;

        assert_eq!(Solution::shuffle(nums.to_vec(), n), output.to_vec());
        assert_eq!(InPlaceSolution::shuffle(nums.to_vec(), n), output.to_vec());
    }
}
