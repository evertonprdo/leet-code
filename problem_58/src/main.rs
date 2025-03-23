// 58. Length of Last Word: https://leetcode.com/problems/length-of-last-word

fn main() {
    println!(
        "{}",
        length_of_last_word(String::from("   fly me   to   the moon  "))
    );
}

fn length_of_last_word(s: String) -> i32 {
    let mut r = 0;
    let mut iter = s.as_bytes().iter().rev();

    while let Some(c) = iter.next() {
        match c {
            32 if r == 0 => continue,
            32 => return r,
            _ => r += 1,
        }
    }
    r
}

// To solve the problem you always can do a "".trim_end() which is almost the same as the above, or...
// the most straightforward that I saw s.split_whitespace().last().len() || 0

/* My first attempt:

    fn length_of_last_word(s: String) -> i32 {
        let mut ignore_wsp = true;
        let mut iter = s.as_bytes().iter().rev();
        let mut r = 0;

        while let Some(c) = iter.next() {
            if ignore_wsp && *c != 32 {
                ignore_wsp = false;
            }

            if !ignore_wsp {
                if *c == 32 {
                    break;
                }
                r += 1;
            }
        }
        r
    }
*/

/* The current solution are based on that solution:

    fn length_of_last_word(s: String) -> i32 {
        let mut counter = 0;
        let s_chars: Vec<char> = s.chars().collect();

        for i in s_chars.iter().rev() {
            match i {
                ' ' if counter == 0 => continue,
                ' ' => return counter,
                _ => counter += 1,
            }
        }

        counter
    }
*/
