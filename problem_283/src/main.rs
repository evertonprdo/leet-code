// 283. Move Zeroes: https://leetcode.com/problems/move-zeroes
fn main() {
    let mut num = vec![0, 1, 0, 3, 12];
    move_zeroes2(&mut num);
    println!("{:?}", num);
}

// My first attempt: new vec; zeros go at the end and non-zeros at the beginning
fn move_zeroes(nums: &mut Vec<i32>) {
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut tmp: Vec<i32> = vec![0; nums.len()];

    for n in nums.iter() {
        match n {
            0 => {
                tmp[r] = *n;
                r -= 1;
            }
            _ => {
                tmp[l] = *n;
                l += 1;
            }
        }
    }

    let _ = std::mem::replace(nums, tmp);
}

// https://leetcode.com/problems/move-zeroes/solutions/172432/the-easiest-but-unusual-snowball-java-solution-beats-100-o-n-clear-explanation/
fn move_zeroes2(nums: &mut Vec<i32>) {
    // `s` counts the number of zeroes encountered so far,
    // giving the offset needed to place the next non-zero at the start of the zero chain.
    let mut s = 0;

    for i in 0..nums.len() {
        if nums[i] == 0 {
            s += 1;
        } else if s > 0 {
            // In practice, this "swaps" the current value with the
            // first zero in the chain (i.e., nums[i - s]);
            nums[i - s] = nums[i];

            // Since the right side is always a zero after the move,
            // direct overwrite is fine — no need for swap().
            nums[i] = 0;
        }
    }
}
