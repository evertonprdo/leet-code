fn main() {
    println!(
        "{}",
        length_of_last_word(String::from("   fly me   to   the moon  "))
    );
}

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
