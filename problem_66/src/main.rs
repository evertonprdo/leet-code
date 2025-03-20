// 66. Plus One: https://leetcode.com/problems/plus-one

fn main() {
    let digits = vec![1, 9, 9, 9];
    println!("{:?}", plus_one(digits))
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut one_was_added = false;

    for d in digits.iter_mut().rev() {
        if one_was_added {
            break;
        };

        *d = match d {
            9 => 0,
            _ => {
                one_was_added = true;
                *d + 1
            }
        };
    }

    if one_was_added {
        return digits;
    }

    digits.insert(0, 1);
    return digits;
}
