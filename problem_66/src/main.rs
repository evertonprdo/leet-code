// 66. Plus One: https://leetcode.com/problems/plus-one

fn main() {
    let digits = vec![1, 9, 9, 9];
    println!("{:?}", plus_one(digits))
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut one_was_added = false;

    for d in digits.iter_mut().rev() {
        if *d == 9 {
            *d = 0;
        } else {
            *d = *d + 1;
            one_was_added = true;

            break;
        }
    }

    if one_was_added {
        return digits;
    }

    digits.insert(0, 1);
    return digits;
}
