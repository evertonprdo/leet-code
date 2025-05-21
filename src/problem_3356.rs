// 3356. Zero Array Transformation II: https://leetcode.com/problems/zero-array-transformation-ii

pub struct Solution {}
impl Solution {
    // https://leetcode.com/problems/zero-array-transformation-ii/solutions/6531047/binary-search-python-c-java-js-c-go/
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut l = 1;
        let mut r = queries.len();

        if nums.iter().all(|&x| x == 0) {
            return 0;
        }

        if !Self::can_make_zero_array(&nums, &queries) {
            return -1;
        }

        while l < r {
            let mid = l + (r - l) / 2;
            if Self::can_make_zero_array(&nums, &queries[..mid]) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l as i32
    }

    fn can_make_zero_array(nums: &Vec<i32>, queries: &[Vec<i32>]) -> bool {
        let n = nums.len();
        let mut diff = vec![0; n];

        for query in queries {
            let s = query[2];
            let r = query[1] as usize;
            let l = query[0] as usize;

            diff[l] += s;
            if r + 1 < n {
                diff[r + 1] -= s;
            }
        }

        let mut curr = 0;
        for i in 0..n {
            curr += diff[i];

            if nums[i] > curr {
                return false;
            }
        }

        true
    }
}

pub struct OtherSolution {}
impl OtherSolution {
    // https://leetcode.com/problems/zero-array-transformation-ii/solutions/6531748/kotlin-rust/
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let (mut k, mut sum) = (0, 0);
        let mut s = vec![0; nums.len() + 1];

        for i in 0..nums.len() {
            sum += s[i];
            while sum < nums[i] {
                if k >= queries.len() {
                    return -1;
                }

                let v = queries[k][2];
                let r = queries[k][1] as usize;
                let l = queries[k][0] as usize;

                s[l] += v;
                s[r + 1] -= v;
                k += 1;

                if (l..=r).contains(&i) {
                    sum += v
                }
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input1 = [2, 0, 2];
        let input2 = [[0, 2, 1], [0, 2, 1], [1, 1, 3]];
        let output = 2;

        let nums = input1.to_vec();
        let queries = input2.map(|x| x.to_vec()).to_vec();

        assert_eq!(Solution::min_zero_array(nums, queries), output);

        let nums = input1.to_vec();
        let queries = input2.map(|x| x.to_vec()).to_vec();

        assert_eq!(OtherSolution::min_zero_array(nums, queries), output);
    }

    #[test]
    fn example_02() {
        let input1 = [4, 3, 2, 1];
        let input2 = [[1, 3, 2], [0, 2, 1]];
        let output = -1;

        let nums = input1.to_vec();
        let queries = input2.map(|x| x.to_vec()).to_vec();

        assert_eq!(Solution::min_zero_array(nums, queries), output);

        let nums = input1.to_vec();
        let queries = input2.map(|x| x.to_vec()).to_vec();

        assert_eq!(OtherSolution::min_zero_array(nums, queries), output);
    }

    #[test]
    fn example_03() {
        let input1 = [5];
        let input2 = [[0, 0, 5], [0, 0, 1], [0, 0, 3], [0, 0, 2]];
        let output = 1;

        let nums = input1.to_vec();
        let queries = input2.map(|x| x.to_vec()).to_vec();

        assert_eq!(Solution::min_zero_array(nums, queries), output);

        let nums = input1.to_vec();
        let queries = input2.map(|x| x.to_vec()).to_vec();

        assert_eq!(OtherSolution::min_zero_array(nums, queries), output);
    }
}
