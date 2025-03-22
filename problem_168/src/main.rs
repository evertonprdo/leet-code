fn main() {
    //let ipt = 18279;
    let ipt = 2147483647;

    println!("{}", convert_to_title(ipt));
}

pub fn convert_to_title(column_number: i32) -> String {
    let mut n = column_number;
    let mut title: Vec<u8> = Vec::new();

    while n > 0 {
        let rest = (n % 26) as u8;
        title.insert(0, if rest == 0 { 90 } else { rest + 64 });

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

    (18279 - 1) / 26 = 703 -> A
    (703   - 1) / 26 = 27  -> A
    (27    - 1) / 26 = 1   -> A
    (1     - 1) / 26 = 0   -> A
*/
