// 171. Excel Sheet Column Number: https://leetcode.com/problems/excel-sheet-column-number

fn main() {
    let column_title = String::from("FXSHRXW");
    println!("{}", title_to_number(column_title));
}

fn title_to_number(column_title: String) -> i32 {
    let mut sum: i32 = 0;
    for b in column_title.bytes() {
        //                      64 = 'A' - 1
        sum = (sum * 26) + (b - 64) as i32
    }
    sum
}

/* My first attempt was to to just implement the formula:
    (a, b, c) = (a × 26²) + (b × 26¹) + (c × 26⁰)

    for (i, b) in column_title.bytes().rev().enumerate() {
        sum += (b as i32 - 64) * 26_i32.pow(i as u32);
    }
*/

/* But I liked this one better:
    https://leetcode.com/problems/excel-sheet-column-number/submissions/1581562116/

    for b in column_title.bytes() {
        sum = (sum * 26) + (b - 64) as i32
    }
*/
