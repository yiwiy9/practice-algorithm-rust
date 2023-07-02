use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }

    let mut ok = true;
    for (i, &s_i) in s.iter().enumerate() {
        if s_i % 5 != 0 {
            ok = false;
        }
        if i != 0 && s_i < s[i - 1] {
            ok = false;
        }
        if 100 > s_i || s_i > 675 {
            ok = false;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
