// 88. Merge Sorted Array: https://leetcode.com/problems/merge-sorted-array

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);

    for num in &nums1 {
        print!("{num} ");
    }
}

// solution by: https://leetcode.com/problems/merge-sorted-array/solutions/6502120/simple-rust-solution/
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k: usize = (m + n - 1) as usize;

    while i >= 0 && j >= 0 {
        if nums1[i as usize] > nums2[j as usize] {
            nums1[k] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k] = nums2[j as usize];
            j -= 1;
        }

        k -= 1;
    }

    while j >= 0 {
        nums1[k] = nums2[j as usize];
        k -= 1;
        j -= 1;
    }
}

//                 k
// [1, 2, 3, 0, 0, 0]

//        i
// [1, 2, 3, 0, 0, 0]

//        j
// [2, 5, 6]

// [1, 2, 3, 0, 0, 0]
