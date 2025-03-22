// 168. Excel Sheet Column Title: https://leetcode.com/problems/excel-sheet-column-title

fn main() {
    let ipt = 2147483647;
    println!("{}", convert_to_title(ipt));
}

const BOUNDARIES: [usize; 6] = [27, 703, 18279, 475255, 12356631, 321272407];
pub fn convert_to_title(column_number: i32) -> String {
    let mut n = column_number;
    let mut s = BOUNDARIES
        .iter()
        .position(|&x| (n as usize) < x)
        .map_or(7, |i| i + 1);

    let mut title: Vec<u8> = vec![0; s];

    while n > 0 {
        let rest = (n % 26) as u8;

        //                           b'Z'        x + b'A' - 1
        title[s - 1] = if rest == 0 { 90 } else { rest + 64 };
        s -= 1;

        n = (n - 1) / 26;
    }

    String::from_utf8(title).unwrap()
}

/* For this solution I reversed this:
    https://github.com/evertonprdo/leet-code/blob/main/problem_171/src/main.rs

    s = (0   * 26) + 1 -> 1     -> A
    s = (1   * 26) + 1 -> 27    -> AA
    s = (27  * 26) + 1 -> 703   -> AAA
    s = (703 * 26) + 1 -> 18279 -> AAAA

    (18279 - 1) / 26 = 703 % 26 -> A
    (703   - 1) / 26 = 27  % 26 -> A
    (27    - 1) / 26 = 1   % 26 -> A
    (1     - 1) / 26 = 0   % 26 -> A
*/

/* Number of characters based on n:
    1         -> 1
    27        -> 2
    703       -> 3
    18279     -> 4
    475255    -> 5
    12356631  -> 6
    321272407 -> 7
*/
