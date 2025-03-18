// 69. Sqrt(x): https://leetcode.com/problems/sqrtx/

fn main() {
    let x = 2147483647;
    let result = my_sqrt(x);
    println!("The rounded-down square root of {x} is {result}");
}

fn my_sqrt(x: i32) -> i32 {
    let square: u32 = x as u32;
    let mut i: u32 = 1;

    loop {
        let sqrt = i * i;
        if sqrt == square {
            return i as i32;
        }

        if sqrt > square {
            return (i - 1) as i32;
        }
        i += 1;
    }
}
