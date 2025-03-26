// 3492. Maximum Containers on a Ship: https://leetcode.com/problems/maximum-containers-on-a-ship

use std::cmp::min;

fn main() {
    println!("{}", max_containers(2, 3, 15));
}

fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
    let cells = n * n;
    let n_weight = max_weight / w;

    min(cells, n_weight)
}

// 2 x 2 -> 4
// 3 / 15 -> 5

// 3 x 3 -> 9
// 5 / 20 -> 4
