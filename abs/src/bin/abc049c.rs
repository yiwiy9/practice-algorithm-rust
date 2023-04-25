use proconio::input;

fn main() {
    input! {
        mut s: String
    }

    let search_strings = ["dream", "dreamer", "erase", "eraser"];
    let mut ok = false;

    // 文字列にありがちなindexが大きく変わる系は、forで回すよりloopでやった方が良さそう
    loop {
        if s.is_empty() {
            ok = true;
            break;
        }

        let mut s_sliced = s.clone();
        for search_str in search_strings.iter() {
            if s.ends_with(search_str) {
                s_sliced = s[..s.len() - search_str.len()].to_string();
            }
        }

        if s_sliced == s {
            break;
        }

        s = s_sliced;
    }

    println!("{}", if ok { "YES" } else { "NO" });
}
