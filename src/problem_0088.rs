// 88. Merge Sorted Array: https://leetcode.com/problems/merge-sorted-array

pub struct Solution {}
impl Solution {
    // solution by: https://leetcode.com/problems/merge-sorted-array/solutions/2838852/rust-o-n-simple-efficient-0ms/
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = n as usize;
        let mut k = i + j;

        while i < k {
            if i > 0 && nums1[i - 1] > nums2[j - 1] {
                nums1[k - 1] = nums1[i - 1];
                i -= 1;
            } else {
                nums1[k - 1] = nums2[j - 1];
                j -= 1;
            }
            k -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6])
    }

    #[test]
    fn example_02() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        let m = 1;
        let n = 0;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1])
    }

    #[test]
    fn example_03() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let m = 0;
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1])
    }
}
