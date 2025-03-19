// 13. Roman to Integer
use std::collections::HashMap;

fn main() {
    let s = String::from("MCMXCIV");
    let result = roman_to_int(s.clone());
    println!("{s}: {result}");
}

fn roman_to_int(s: String) -> i32 {
    let roman_map = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let chars: Vec<char> = s.chars().collect();
    let mut result: i32 = 0;

    for i in 0..chars.len() {
        let curr = roman_map[&chars[i]];

        if let Some(k) = chars.get(i + 1) {
            if curr >= roman_map[k] {
                result += curr;
            } else {
                result -= curr;
            }
        } else {
            result += curr;
        }
    }

    return result;
}

//  i
//   ->|
// [M, C, M, X, C, I, V]
