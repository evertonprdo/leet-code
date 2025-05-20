// 3355. Zero Array Transformation I: https://leetcode.com/problems/zero-array-transformation-i

pub struct Solution {}
impl Solution {
    // Time limit exceed
    pub fn is_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        for query in queries {
            let ini = query[0] as usize;
            let end = query[1] as usize;

            for i in ini..=end {
                nums[i] -= 1;
            }
        }

        !nums.iter().any(|&x| x > 0)
    }
}

pub struct DiffSolution {}
impl DiffSolution {
    // https://leetcode.com/problems/zero-array-transformation-i/solutions/6761307/difference-array-with-images-example-walkthrough-c-python-java
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut diff = vec![0i32; n + 1];

        for query in queries {
            let l = query[0] as usize;
            let r = query[1] as usize;

            diff[l] += 1;
            diff[r + 1] -= 1;
        }

        nums.iter()
            .zip(diff.into_iter())
            .scan(0i32, |sum, (&num, delta)| {
                *sum += delta;
                Some(num <= *sum)
            })
            .all(|ok| ok)
    }
}

pub struct UnsafeSolution {}
impl UnsafeSolution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        unsafe {
            let n = nums.len();
            let mut diff = vec![0i32; n + 1];

            for query in queries {
                let l = *query.get_unchecked(0) as usize;
                let r = *query.get_unchecked(1) as usize;

                *diff.get_unchecked_mut(l) += 1;
                *diff.get_unchecked_mut(r + 1) -= 1;
            }

            let mut cnt = 0i32;
            let nums_ptr = nums.as_ptr();
            let diff_ptr = diff.as_ptr();

            for i in 0..n {
                cnt += *diff_ptr.add(i);

                if *nums_ptr.add(i) > cnt {
                    return false;
                }
            }

            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_01() {
        let input1 = [1, 0, 1];
        let input2 = [[0, 2]];

        let nums = input1.to_vec();
        let queries: Vec<Vec<i32>> = input2.map(|x| x.to_vec()).to_vec();

        assert!(Solution::is_zero_array(nums, queries));

        let nums = input1.to_vec();
        let queries: Vec<Vec<i32>> = input2.map(|x| x.to_vec()).to_vec();

        assert!(DiffSolution::is_zero_array(nums, queries));

        let nums = input1.to_vec();
        let queries: Vec<Vec<i32>> = input2.map(|x| x.to_vec()).to_vec();

        assert!(UnsafeSolution::is_zero_array(nums, queries));
    }

    #[test]
    fn example_02() {
        let input1 = [4, 3, 2, 1];
        let input2 = [[1, 3], [0, 2]];

        let nums = input1.to_vec();
        let queries: Vec<Vec<i32>> = input2.map(|x| x.to_vec()).to_vec();

        assert!(!Solution::is_zero_array(nums, queries));

        let nums = input1.to_vec();
        let queries: Vec<Vec<i32>> = input2.map(|x| x.to_vec()).to_vec();

        assert!(!DiffSolution::is_zero_array(nums, queries));

        let nums = input1.to_vec();
        let queries: Vec<Vec<i32>> = input2.map(|x| x.to_vec()).to_vec();

        assert!(!UnsafeSolution::is_zero_array(nums, queries));
    }
}
