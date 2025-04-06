// 35. Search Insert Position: https://leetcode.com/problems/search-insert-position

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 7;

    println!("{}", search_insert(nums, target));
}

// Binary Search
// Time complexity: O(log n)
// Space complexity: O(1)
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let i = l + (r - l) / 2;

        match target.cmp(&nums[i]) {
            std::cmp::Ordering::Equal => return i as i32,

            std::cmp::Ordering::Greater => l = i + 1,
            std::cmp::Ordering::Less => r = i,
        }
    }

    l as i32
}
