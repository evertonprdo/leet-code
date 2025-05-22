// 1920. Build Array from Permutation: https://leetcode.com/problems/build-array-from-permutation

pub struct Solution {}
impl Solution {
    // Time Complexity: O(n)
    // Space Complexity: O(n)
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            ans.push(nums[nums[i] as usize]);
        }
        ans
    }
}

pub struct ModuloSolution {}
impl ModuloSolution {
    // https://leetcode.com/problems/build-array-from-permutation/solutions/6718380/o-n-o-1-space-with-images-example-walkthrough-c-python-java/
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let k = nums[i] as usize;
            let n = nums[k] % 1024;
            let n = n << 10;

            nums[i] += n;
        }

        for i in 0..nums.len() {
            nums[i] = nums[i] >> 10;
        }

        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let nums = [0, 2, 1, 5, 3, 4];
        let output = [0, 1, 2, 4, 5, 3];

        assert_eq!(Solution::build_array(nums.to_vec()), output.to_vec());
        assert_eq!(ModuloSolution::build_array(nums.to_vec()), output.to_vec());
    }

    #[test]
    fn example_02() {
        let nums = [5, 0, 1, 2, 3, 4];
        let output = [4, 5, 0, 1, 2, 3];

        assert_eq!(Solution::build_array(nums.to_vec()), output.to_vec());
        assert_eq!(ModuloSolution::build_array(nums.to_vec()), output.to_vec());
    }
}
