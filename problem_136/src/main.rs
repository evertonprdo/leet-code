// 136. Single Number: https://leetcode.com/problems/single-number

fn main() {
    println!("{}", single_number(vec![4, 1, 2, 1, 2]));
}

// https://leetcode.com/problems/single-number/solutions/3801367/video-single-number-a-bitwise-magic-trick/
// The XOR operator and the property of canceling duplicates x ^ x = 0
fn single_number(nums: Vec<i32>) -> i32 {
    let mut r = 0;
    for n in nums {
        r ^= n
    }
    r
}

/*  First Attempt: HashSet To eliminate duplicates

    fn single_number(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut set: HashSet<i32> = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
                continue;
            }

            set.insert(n);
        }

        *set.iter().next().unwrap()
    }
*/
