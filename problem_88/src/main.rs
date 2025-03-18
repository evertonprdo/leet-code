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

// solution by: https://leetcode.com/problems/merge-sorted-array/solutions/2838852/rust-o-n-simple-efficient-0ms/
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
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

//                 |<-k
// [1, 2, 3, 0, 0, 0]

//        |<-i
// [1, 2, 3, 0, 0, 0]

//        |<-j
// [2, 5, 6]

// [1, 2, 3, 0, 0, 0]
