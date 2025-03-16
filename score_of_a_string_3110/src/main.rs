fn main() {
    let s = "hello";
    let result = score_of_string(String::from(s));
    println!("input: s = \"{s}\"\noutput: {result}")
}

fn score_of_string(s: String) -> i32 {
    let bytes = s.as_bytes();

    let mut curr = 0;
    let mut next = 1;
    let mut sum: i32 = 0;

    while next < bytes.len() {
        sum += ((bytes[curr] as i32) - (bytes[next] as i32)).abs();

        curr += 1;
        next += 1;
    }

    return sum;
}
