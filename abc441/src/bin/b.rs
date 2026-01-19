use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        q: usize,
        w: [Chars; q],
    }

    let is_ok =
        |target: &[char], dict: &[char]| -> bool { target.iter().all(|c| dict.contains(c)) };

    for w_i in &w {
        let is_t = is_ok(w_i, &s);
        let is_a = is_ok(w_i, &t);

        if is_t && is_a {
            println!("Unknown");
        } else if is_t {
            println!("Takahashi");
        } else if is_a {
            println!("Aoki");
        } else {
            println!("Unknown");
        }
    }
}
