// 2894. Divisible and Non-divisible Sums Difference: https://leetcode.com/problems/divisible-and-non-divisible-sums-difference

fn main() {
    let n: i32 = 5;
    let m: i32 = 1;

    let result = difference_of_sums(n, m);
    println!("{result}");
}

fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;

    for i in 1..=n {
        if i % m == 0 {
            num2 += i;
        } else {
            num1 += i;
        }
    }

    return num1 - num2;
}
